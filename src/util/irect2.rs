use super::*;

#[derive(Debug, Clone, Copy)]
pub struct IRect2 {
    pub xy: IVec2,
    pub wh: IVec2,
}

impl IRect2 {
    pub fn overlaps(&self, other: &IRect2) -> bool {
        !(
            self.xy.x + self.wh.x <= other.xy.x ||
            self.xy.x > other.xy.x + other.wh.x ||
            
            self.xy.y + self.wh.y <= other.xy.y ||
            self.xy.y > other.xy.y + other.wh.y
        )
    }
}

pub fn irect(x: i32, y: i32, w: i32, h: i32) -> IRect2 {
    IRect2 {
        xy: ivec2(x,y),
        wh: ivec2(w,h),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps_true() {
        assert!(irect(100,100,100,100).overlaps(&irect(50,50,51,51)));
        assert!(!irect(100,100,100,100).overlaps(&irect(250,50,151,51)));
        assert!(!irect(100,100,100,100).overlaps(&irect(50,250,51,151)));
        assert!(irect(0,0,768,64).overlaps(&irect(760,5,10,10)));
        assert!(irect(0,0,768,64).overlaps(&irect(760,56,10,10)));
        assert!(!irect(-10,0,768,64).overlaps(&irect(760,56,10,10)));
        assert!(irect(-10,0,768,64).overlaps(&irect(730,56,10,10)));
        assert!(!irect(0,0,768,64).overlaps(&irect(760,65,10,10)));
        let rect1 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect2 = IRect2 { xy: IVec2 { x: 2, y: 2 }, wh: IVec2 { x: 4, y: 4 } };
        assert!(rect1.overlaps(&rect2));

        let rect3 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect4 = IRect2 { xy: IVec2 { x: 3, y: 3 }, wh: IVec2 { x: 4, y: 4 } };
        assert!(rect3.overlaps(&rect4));
    }

    #[test]
    fn test_overlaps_false() {
        let rect1 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 2, y: 2 } };
        let rect2 = IRect2 { xy: IVec2 { x: 3, y: 3 }, wh: IVec2 { x: 2, y: 2 } };
        assert!(!rect1.overlaps(&rect2));

        let rect3 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect4 = IRect2 { xy: IVec2 { x: 5, y: 5 }, wh: IVec2 { x: 2, y: 2 } };
        assert!(!rect3.overlaps(&rect4));
    }

    #[test]
    fn test_overlaps_touching_edges() {
        let rect1 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect2 = IRect2 { xy: IVec2 { x: 4, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        assert!(rect1.overlaps(&rect2));

        let rect3 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect4 = IRect2 { xy: IVec2 { x: 0, y: 4 }, wh: IVec2 { x: 4, y: 4 } };
        assert!(rect3.overlaps(&rect4));

        let rect5 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect6 = IRect2 { xy: IVec2 { x: 4, y: 4 }, wh: IVec2 { x: 4, y: 4 } };
        assert!(rect5.overlaps(&rect6));
    }

    #[test]
    fn test_overlaps_inside() {
        let rect1 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect2 = IRect2 { xy: IVec2 { x: 1, y: 1 }, wh: IVec2 { x: 2, y: 2 } };
        assert!(rect1.overlaps(&rect2));

        let rect3 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect4 = IRect2 { xy: IVec2 { x: 1, y: 1 }, wh: IVec2 { x: 1, y: 1 } };
        assert!(rect3.overlaps(&rect4));
    }

    #[test]
    fn test_overlaps_corner_cases() {
        let rect1 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect2 = IRect2 { xy: IVec2 { x: 4, y: 4 }, wh: IVec2 { x: 4, y: 4 } };
        assert!(rect1.overlaps(&rect2));

        let rect3 = IRect2 { xy: IVec2 { x: 0, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        let rect4 = IRect2 { xy: IVec2 { x: 4, y: 0 }, wh: IVec2 { x: 4, y: 4 } };
        assert!(rect3.overlaps(&rect4));
    }
}
