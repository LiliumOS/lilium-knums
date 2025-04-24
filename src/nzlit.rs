macro_rules! nzlit {
    ($lit:literal) => {
        const {
            let val = $lit;
            assert!(val != 0, "Cannot construct NonZero with 0 value");
            unsafe { ::core::num::NonZero::new_unchecked(val) }
        }
    };
}

pub(crate) use nzlit;
