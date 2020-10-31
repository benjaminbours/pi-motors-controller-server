use rppal::gpio::{Error, Gpio};

const GPIO_LEFT_MOTOR_1: u8 = 7;
const GPIO_LEFT_MOTOR_2: u8 = 8;
const GPIO_RIGHT_MOTOR_1: u8 = 9;
const GPIO_RIGHT_MOTOR_2: u8 = 10;

pub struct MotorsController {
    left_motor_pins: (rppal::gpio::OutputPin, rppal::gpio::OutputPin),
    right_motor_pins: (rppal::gpio::OutputPin, rppal::gpio::OutputPin),
}

impl MotorsController {
    pub fn new() -> Result<MotorsController, Error> {
        Ok(MotorsController {
            left_motor_pins: (
                Gpio::new()?.get(GPIO_LEFT_MOTOR_1)?.into_output(),
                Gpio::new()?.get(GPIO_LEFT_MOTOR_2)?.into_output(),
            ),
            right_motor_pins: (
                Gpio::new()?.get(GPIO_RIGHT_MOTOR_1)?.into_output(),
                Gpio::new()?.get(GPIO_RIGHT_MOTOR_2)?.into_output(),
            ),
        })
    }

    pub fn forward(&mut self) {
        println!("forward");
        let (pin_left_1, pin_left_2) = &mut self.left_motor_pins;
        let (pin_right_1, pin_right_2) = &mut self.right_motor_pins;
        pin_left_1.set_low();
        pin_left_2.set_high();
        pin_right_1.set_low();
        pin_right_2.set_high();
    }

    pub fn backward(&mut self) {
        println!("backward");
        let (pin_left_1, pin_left_2) = &mut self.left_motor_pins;
        let (pin_right_1, pin_right_2) = &mut self.right_motor_pins;
        pin_left_1.set_high();
        pin_left_2.set_low();
        pin_right_1.set_high();
        pin_right_2.set_low();
    }

    pub fn left(&mut self) {
        println!("left");
        let (pin_left_1, pin_left_2) = &mut self.left_motor_pins;
        let (pin_right_1, pin_right_2) = &mut self.right_motor_pins;
        pin_left_1.set_low();
        pin_left_2.set_high();
        pin_right_1.set_high();
        pin_right_2.set_low();
    }

    pub fn right(&mut self) {
        println!("right");
        let (pin_left_1, pin_left_2) = &mut self.left_motor_pins;
        let (pin_right_1, pin_right_2) = &mut self.right_motor_pins;
        pin_left_1.set_high();
        pin_left_2.set_low();
        pin_right_1.set_low();
        pin_right_2.set_high();
    }

    pub fn stop(&mut self) {
        println!("stop");
        let (pin_left_1, pin_left_2) = &mut self.left_motor_pins;
        let (pin_right_1, pin_right_2) = &mut self.right_motor_pins;
        pin_left_1.set_high();
        pin_left_2.set_high();
        pin_right_1.set_high();
        pin_right_2.set_high();
    }
}
