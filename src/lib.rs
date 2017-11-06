use std::ops::Range;

pub fn new(x:Range<i32>, y:Range<i32>) -> Vec<(i32, i32)> {

    let mut xy = Vec::<(i32, i32)>::new();

    for j in y.start..y.end + 1 {

        for i in x.start..x.end + 1 {

            xy.push((i, j));

        }

    }

    xy

}
