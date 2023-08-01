use crate::consts::TOPVAL;
use crate::error::TxLshError;

use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::{Add, Sub};

pub(crate) const BUCKET_SIZE: usize = 256;
/// Size of a sliding window to process a byte string and populate an array of bucket counts.
pub(crate) const WINDOW_SIZE: usize = 5;

static mut BIT_PAIRS_FLAG: bool = false;
static mut BIT_PAIRS_DIFF: [[usize; 256]; 256] = [[0; 256]; 256];

/// enums

/// An enum determining the number of buckets for hashing.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum BucketKind {
    /// Hashing with 128 buckets.
    Bucket128,
    /// Hashing with 256 buckets. "Full hash"
    Bucket256,
}

impl BucketKind {
    /// Returns the number of buckets.
    pub fn bucket_count(&self) -> usize {
        match self {
            BucketKind::Bucket128 => 128,
            BucketKind::Bucket256 => 256,
        }
    }
}

/// An enum determining the length of checksum.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ChecksumKind {
    /// TxLsh uses one byte for checksum. The collision rate is 1/24.
    OneByte,
    /// TxLsh uses three bytes for checksum. The collision rate is 1/5800.
    ThreeByte,
}

impl ChecksumKind {
    pub fn checksum_len(&self) -> usize {
        match self {
            ChecksumKind::OneByte => 1,
            ChecksumKind::ThreeByte => 3,
        }
    }
}

/// An enum representing the version of TxLsh.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Version {
    /// Original Tlsh, mapping to an empty string ```""```.
    Original,
    /// Current Tlsh, mapping to an string ```"T1"```.
    Version4,
    /// Original TxLsh, mapping to an string ```"X1"```.
    TxLshV1,
}

impl Version {
    pub fn ver(&self) -> &str {
        match self {
            Version::Original => "",
            Version::Version4 => "T1",
            Version::TxLshV1 => "X1",
        }
    }
}

pub(crate) fn hash_len(bucket: BucketKind, checksum: ChecksumKind, ver: Version) -> usize {
    (bucket.bucket_count() >> 1) + (checksum.checksum_len() << 1) + ver.ver().len() + 4
}

pub(crate) fn find_quartiles(buckets: &[u32], bucket_count: usize) -> (u32, u32, u32) {
    let mut buckets_copy: Vec<u32> = buckets[0..bucket_count].to_vec();
    let (mut shortcut_low, mut shortcut_high) = (vec![0; bucket_count], vec![0; bucket_count]);
    let (mut spl, mut sph) = (0, 0);

    let quartile = bucket_count >> 2;
    let p1 = quartile - 1;
    let p2 = p1 + quartile;
    let p3 = p2 + quartile;
    let end = p3 + quartile;

    // Applies quicksort to find p2
    let (mut low, mut high) = (0, end);
    let q2 = loop {
        let pivot = partition(&mut buckets_copy, low, high);

        match pivot.cmp(&p2) {
            Greater => {
                high = pivot - 1;
                shortcut_high[sph] = pivot;
                sph += 1;
            }
            Less => {
                low = pivot + 1;
                shortcut_low[spl] = pivot;
                spl += 1;
            }

            Equal => {
                break buckets_copy[p2];
            }
        }
    };

    shortcut_low[spl] = p2 - 1;
    shortcut_high[sph] = p2 + 1;

    let mut q1 = 0;
    low = 0;
    //for ii in 0..spl {
    for item in shortcut_low.iter().take(spl) {
        high = *item;

        match high.cmp(&p1) {
            Greater => {
                q1 = loop {
                    let pivot = partition(&mut buckets_copy, low, high);
                    match pivot.cmp(&p1) {
                        Greater => high = pivot - 1,
                        Less => low = pivot + 1,
                        Equal => break buckets_copy[p1],
                    }
                };
                break;
            }
            Less => {
                low = high;
            }
            Equal => {
                q1 = buckets_copy[p1];
                break;
            }
        }
    }

    let mut q3 = 0;
    high = end;
    for item in shortcut_high.iter().take(sph) {
        low = *item;
        match low.cmp(&p3) {
            Less => {
                q3 = loop {
                    let pivot = partition(&mut buckets_copy, low, high);
                    match pivot.cmp(&p3) {
                        Less => low = pivot + 1,
                        Greater => high = pivot - 1,
                        Equal => break buckets_copy[p3],
                    }
                };
                break;
            }

            Equal => {
                q3 = buckets_copy[p3];
                break;
            }

            Greater => high = low,
        }
    }

    (q1, q2, q3)
}

pub(crate) fn partition(buckets: &mut [u32], low: usize, high: usize) -> usize {
    if low == high {
        return low;
    }

    if low + 1 == high {
        if buckets[low] > buckets[high] {
            buckets.swap(low, high);
        }

        return low;
    }

    let (mut result, pivot) = (low, (low + high) >> 1);
    let val = buckets[pivot];
    buckets.swap(pivot, high);

    for ii in low..high {
        if buckets[ii] < val {
            buckets.swap(ii, result);
            result += 1;
        }
    }

    buckets[high] = buckets[result];
    buckets[result] = val;

    result
}

pub(crate) fn l_capturing(len: usize) -> Result<usize, TxLshError> {
    let (mut top, mut bottom) = (TOPVAL.len(), 0);
    let mut idx = top >> 1;

    while idx < TOPVAL.len() {
        if idx == 0 {
            return Ok(idx);
        }

        if len <= TOPVAL[idx] && len > TOPVAL[idx - 1] {
            return Ok(idx);
        }

        if len < TOPVAL[idx] {
            top = idx - 1;
        } else {
            bottom = idx + 1;
        }

        idx = (bottom + top) >> 1;
    }

    Err(TxLshError::DataLenOverflow)
}

pub(crate) fn mod_diff<T>(x: T, y: T, circ_q: T) -> T
where
    T: Copy + PartialEq + Ord + Add<Output = T> + Sub<Output = T>,
{
    let (dl, dr) = if x >= y {
        (x - y, y + circ_q - x)
    } else {
        (y - x, x + circ_q - y)
    };

    std::cmp::min(dl, dr)
}

pub(crate) fn bit_distance(x: &[u8], y: &[u8]) -> usize {
    let mut result = 0;

    for ii in 0..x.len() {
        unsafe {
            result += bit_pairs_diff(x[ii] as usize, y[ii] as usize);
        }
    }

    result
}

#[inline]
unsafe fn bit_pairs_diff(row: usize, col: usize) -> usize {
    let f = |x: &mut i16, y: &mut i16, diff: &mut i16| {
        let d = (*x % 4 - *y % 4).abs();
        *diff += if d == 3 { 6 } else { d };

        *x /= 4;
        *y /= 4;
    };

    if !BIT_PAIRS_FLAG {
        for ii in 0..256i16 {
            for jj in 0..256 {
                let (mut x, mut y, mut diff) = (ii, jj, 0);
                for _ in 0..4 {
                    f(&mut x, &mut y, &mut diff);
                }

                BIT_PAIRS_DIFF[ii as usize][jj as usize] = diff as usize;
            }
        }
        BIT_PAIRS_FLAG = true;
    }

    BIT_PAIRS_DIFF[row][col]
}
