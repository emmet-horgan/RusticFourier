use ndarray::prelude::*;

pub fn cross_correlation() {
    std::unimplemented!();
}

pub fn auto_correlation() {
    std::unimplemented!();
}

pub fn peak_detection() {
    std::unimplemented!();
}

pub fn fourer_series() {
    std::unimplemented!();
}

pub fn hpd() {
    std::unimplemented!();
}

pub fn zero_crossing() {
    std::unimplemented!();
}

impl<T> crate::traits::Stats for Array1<T> {
    type Elements = T;
    fn mean(&self) -> T {
        std::unimplemented!();
    }

    fn variance(&self) -> T {
        std::unimplemented!();
    }
    
    fn stdev(&self) -> T {
        std::unimplemented!();
    }

    fn skewness(&self) -> T {
        std::unimplemented!();
    }
    
    fn kurtosis(&self) -> T {
        std::unimplemented!();
    }

    fn histogram(&self) {
        std::unimplemented!();
    }
    
}


#[cfg(test)]
mod tests {

    #[test]
    fn cross_correlation() {
        std::unimplemented!();
    }
    
    #[test]
    fn auto_correlation() {
        std::unimplemented!();
    }
    
    #[test]
    fn peak_detection() {
        std::unimplemented!();
    }
    
    #[test]
    fn fourer_series() {
        std::unimplemented!();
    }
    
    #[test]
    fn hpd() {
        std::unimplemented!();
    }
    
    #[test]
    fn zero_crossing() {
        std::unimplemented!();
    }
    
}