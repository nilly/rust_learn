//reference: https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}
//move semantic
impl IntoIterator for Pixel {
    type Item = i8;
    type IntoIter = PixelIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PixelIntoIterator {
            pixel: self,
            index: 0,
        }
    }
}

struct PixelIntoIterator {
    pixel: Pixel,
    index: usize,
}

impl Iterator for PixelIntoIterator {
    type Item = i8;
    fn next(&mut self) -> Option<i8> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

//ref semantic 
impl<'a> IntoIterator for &'a Pixel {
    type Item = i8;
    type IntoIter = PixelIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterator {
            pixel: self, //注意self的具体类型为：&'a Pixel
            index: 0,
        }
    }
}

struct PixelIterator<'a> {
    pixel: &'a Pixel,
    index: usize,
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = i8;
    fn next(&mut self) -> Option<i8> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

//ref mut semantic
impl<'a> IntoIterator for &'a mut Pixel {
    type Item = &'a mut i8;
    type IntoIter = PixelMutIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PixelMutIterator {
            pixel: self, //注意self的具体类型为：&'a Pixel
            index: 0,
        }
    }
}
struct PixelMutIterator<'a> {
    pixel: &'a mut Pixel,
    index: usize,
}
impl<'a> Iterator for PixelMutIterator<'a> {
    type Item =  &'a mut i8;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => &mut self.pixel.r,
            1 => &mut self.pixel.g,
            2 => &mut self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

//test
fn main() {
    let mut p = Pixel {
        r: 54,
        g: 23,
        b: 74,
    };
    //ref mut semantic test case. 
    for c in &mut p {
        println!("ref mut semantic: {}", c);
    }
    //ref semantic test case.
    for c in &p {
        println!("ref semantic: {}", c);
    }
    //move semantic test case.
    for component in p {//for 自动调用into_iter获得迭代器， 同时p的所有权也被move进PixelIntoIterator了。
        println!("move semantic: {}", component);
    }
    //后面代码不能再访问p了，因为它已经丧失所有权，失效了。
}