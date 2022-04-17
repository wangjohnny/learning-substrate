use crate::lession4::demo3::Triangle;
use crate::lession4::demo3::Circle;
use crate::lession4::demo3::print_shape_area;
use crate::lession4::demo3::Rectangle;
use crate::lession4::demo1::TrafficLightTime;

fn main() {

    // demo1
    let traffic_light_panel = lession4::demo1::TrafficLightPanel {};
    println!("Red light time: {}", traffic_light_panel.time(lession4::demo1::TrafficLightKind::Red));
    println!("Green light time: {}", traffic_light_panel.time(lession4::demo1::TrafficLightKind::Green));
    println!("Yellow light time: {}", traffic_light_panel.time(lession4::demo1::TrafficLightKind::Yellow));
    
    // demo2
    let v_1 = vec![1_u32, 2_u32, 3_u32];
    let total_1 = lession4::demo2::sum(&v_1);
    
    let v_2 = vec![1_u32, 2_u32, 3_u32, u32::MAX];
    let total_2 = lession4::demo2::sum(&v_2);
    
    println!("Demo 2 of lession 4: ");
    println!("right result: {:?}, overflow result: {:?}", total_1, total_2);
    
    //demo3
    let r = Rectangle {
        width: 10_f64,
        height: 20_f64,
    };
    
    print_shape_area(&r);
    
    let c = Circle {
        radius: 10_f64,
    };
    
    print_shape_area(&c);
    
    let t = Triangle {
        base: 10_f64,
        height: 20_f64,
    };
    
    print_shape_area(&t);
}

mod lession4 {

    pub mod demo1 {
        pub enum TrafficLightKind {
            Red,
            Green,
            Yellow,
        }
        
        pub struct TrafficLightPanel {}
        
        pub trait TrafficLightTime {
            fn time(&self, light_kind: TrafficLightKind) -> u8;
        }
        
        impl TrafficLightTime for TrafficLightPanel {
            fn time(&self, light_kind: TrafficLightKind) -> u8 {
                match light_kind {
                    TrafficLightKind::Red => 60,
                    TrafficLightKind::Green => 50,
                    TrafficLightKind::Yellow => 40,
                }
            }
        }
    }
    
    pub mod demo2 {
        pub fn sum(v: &[u32]) -> Option<u32> {
            let mut num = 0_u32;
            for i in v {
                let (n, b) = num.overflowing_add(*i);
                if b {
                    return None
                } else {
                    num = n;
                }
            }
            Some(num)
        }
    }
    
    pub mod demo3 {
        pub trait ShapeArea {
            fn area(&self) -> f64;
        }
        
        #[derive(Debug)]
        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }

        impl ShapeArea for Rectangle {
            fn area(&self) -> f64 {
                self.width * self.height
            }
        }
        
        #[derive(Debug)]
        pub struct Circle {
            pub radius: f64,
        }

        impl ShapeArea for Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius.powf(2.0)
            }
        }
        
        #[derive(Debug)]
        pub struct Triangle {
            pub base: f64,
            pub height: f64,
        }
        
        impl ShapeArea for Triangle {
            fn area(&self) -> f64 {
                (self.base * self.height) / 2.0
            }
        }
        
        pub fn print_shape_area<T: ShapeArea + std::fmt::Debug>(s: &T) {
            println!("The area of shape: {:?} is {}", s, s.area())
        }
    }
}
