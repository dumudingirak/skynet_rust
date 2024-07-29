pub struct BenchmarkResults{
    pub median:u128,
    pub average:f64,
    pub minimum:u128,
    pub maximum:u128
}

impl BenchmarkResults{
    pub fn new(durations:Vec<u128>)-> BenchmarkResults{
        return BenchmarkResults{
            median: Self::median(&durations),
            average: Self::average(&durations),
            minimum: Self::min(&durations),
            maximum: Self::max(&durations)
        };
    }

    pub fn print(&self, num_iterations:usize) -> (){
        println!("For {} Iterations of skynet:
              Median: \t\t {} ns,
              Average: \t\t {} ns,
              Minimum: \t\t {} ns,
              Maximum: \t\t {} ns", 
              num_iterations, 
              self.median as f64/1_000_000.0,
              self.average/1_000_000.0,
              self.minimum as f64/1_000_000.0,
              self.maximum as f64/1_000_000.0);
    }

    fn median(vector:&Vec<u128>) -> u128{
        let length = vector.len();
        if length % 2 == 0{
            (vector[length/2] + vector[length/2 - 1])/2
        }
        else{
            vector[length/2]
        }
    }

    fn average(vector:&Vec<u128>) -> f64{
        let mut sum:u128 = 0;
        for int in vector{
            sum += *int;
        }
        sum as f64/vector.len() as f64
    }

    fn min(vector:&Vec<u128>) -> u128{
        let mut minimum = vector[0];
        for int in vector{
            if minimum > *int{
                minimum = *int;
            }
        }
        minimum
    }

    fn max(vector:&Vec<u128>) -> u128{
        let mut maximum = vector[0];
        for int in vector{
            if maximum < *int{
                maximum = *int;
            }
        }
        maximum
    }
}