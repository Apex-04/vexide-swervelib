
/* ------------------------------------------------ */
/*                                                  */
/*    Project: vexide-swervelib                     */
/*    File: swervedrive.rs                          */
/*    Author: Andrew Bobay                          */
/*    Date Created: Sep 23rd 2025 12:00PM           */
/*    Date Modified: Nov 7th 2025 06:45PM           */
/*    Team: BBR1                                    */
/*    Description: swerveib modules                 */
/*                                                  */
/* ------------------------------------------------ */
use vexide::prelude::*;
use alloc::vec;
use alloc::vec::Vec;
use vexide::devices::controller::ControllerState;

use crate::swervelib;


pub struct SwerveDrive{
    left_module: swervelib::swervemod::SwerveModule,
    right_module: swervelib::swervemod::SwerveModule,
    inertial: InertialSensor,

}

impl SwerveDrive{
    pub fn new(left_module: swervelib::swervemod::SwerveModule, right_module: swervelib::swervemod::SwerveModule, inertial:InertialSensor) -> Self{
        Self{left_module, right_module, inertial}
    }

    pub fn drive(controller_state: ControllerState){

    }

    pub fn drive_to_coordinates(&mut self, mut splines: Vec<swervelib::spline::Spline>){
        for spline in splines.iter_mut(){

        }
    }

}