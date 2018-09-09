mod integrate {
    /// Computes an integral using the trapezoid rule. 
    /// 
    /// Assumptions: 
    /// 1. That the data is evenly-spaced.
    pub fn trapezoidal(data: &[(f64,f64)]) -> f64 {
        if data.len() <= 1 {
            return 0.0;
        }

        let last = data.len() - 1;
        let h = data[1].0 - data[0].0;
        let mut result = 0.5*(data[0].1 + data[last].1);

        for x in 1..last {
            result += data[x].1;
        }

        result *= h;
        result
    }

    /// Computes an integral using Simpson's rule. 
    /// 
    /// Assumptions: 
    /// 1. That the data is evenly-spaced.
    /// 2. That there are an odd number of data points (even number of slices).
    /// 3. That there are 3 or more data points.
    pub fn simpson(data: &[(f64,f64)]) -> f64 {
        if data.len() <= 2 {
            return 0.0;
        }

        if data.len() % 2 == 0 {
            return 0.0;
        }

        let last = data.len() - 1;
        let h = data[1].0 - data[0].0;
        let mut result = 0.0;

        result += data[0].1;
        result += data[last].1;

        let mut subres4 = 0.0;
        let num_odd = data.len() / 2;
        for x in 0..num_odd {
            let i = 2*x + 1;
            subres4 += data[i].1
        }
        result += 4.0*subres4;

        let mut subres2 = 0.0;
        let num_even = num_odd - 1;
        for x in 0..num_even {
            let i = 2*x + 2;
            subres2 += data[i].1;
        }
        result += 2.0*subres2;

        result *= h/3.0;
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_trapezoidal_empty_data() {
            // Empty data
            let data: [(f64,f64); 0] = [];

            let expected = 0.0;
            let actual = trapezoidal(&data);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_trapezoidal_one_element() {
            // Empty data
            let data: [(f64,f64); 1] = [(0.0, 1.0)];

            let expected = 0.0;
            let actual = trapezoidal(&data);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_trapezoidal_two_elements() {
            let data = [(0.0, 0.0), (1.0, 1.0)];

            let expected = 0.5;
            let actual = trapezoidal(&data);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_trapezoidal_sin_squared() {
            // This data represents sin^2(x)
            let data = [
                (0.0,  0.0), 
                (0.25, 0.0612087), 
                (0.5,  0.229849), 
                (0.75, 0.464631), 
                (1.0,  0.708073)];

            // Incidentaly, real answer closer to 0.27268
            let expected = 0.2774313;
            let actual = trapezoidal(&data);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_simpson_sin_squared() {
            // This data represents sin^2(x)
            let data = [
                (0.0,  0.0), 
                (0.25, 0.0612087), 
                (0.5,  0.229849), 
                (0.75, 0.464631), 
                (1.0,  0.708073)];

            // Incidentaly, real answer closer to 0.27268
            let expected = 0.27259415;
            let actual = simpson(&data);

            assert_eq!(expected, actual);
        }
    }
}