macro_rules! return_ok_if_some {
    ( $e:expr ) => {
        match $e {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {}
        }
    };
}

macro_rules! return_if_some {
    ( $e:expr ) => {
        if let Some(_) = $e {
            return $e;
        }
    };
}

pub(crate) use return_if_some;
pub(crate) use return_ok_if_some;
