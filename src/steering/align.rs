use crate::domain::{steering::{Behaviour, Steering}, kinematic::Kinematic};
use crate::vector::Vector;

pub struct Align {}

impl Behaviour for Align {
  type Args = f32;

  fn calculate(self: &Self, kinematic: Kinematic, args: Self::Args) -> Steering {

    Steering {
      angular: 0.0,
      linear: Vector(0.0, 0.0)
    }
  }

}

// export default class Align extends AbstractBehaviour {
//   readonly name = "ALIGN";
//   targetId: CharacterId;
//   maxRotation: number;
//   decelerationTolerance: number;
//   alignTolerance: number;
//   timeToTarget: number;
//
//   constructor(
//     targetId: CharacterId,
//     maxRotation?: number,
//     decelerationTolerance?: number,
//     alignTolerance?: number,
//     timeToTarget?: number
//   ) {
//     super();
//     this.targetId = targetId;
//     this.maxRotation = maxRotation ?? 120;
//     this.decelerationTolerance = decelerationTolerance ?? 2;
//     this.alignTolerance = alignTolerance ?? 0.01;
//     this.timeToTarget = timeToTarget ?? 0.1;
//   }
//
//   calculate(kinematic: Kinematic, orientation: number): Steering {
//     const linear: Vector = [0, 0];
//
//     const rotation = mapToRange(orientation - kinematic.orientation);
//     const rotationSize = Math.abs(rotation);
//
//     if (rotationSize < this.alignTolerance) {
//       return {
//         linear,
//         angular: 0,
//       };
//     }
//
//     const isSlowed = rotationSize <= this.decelerationTolerance;
//
//     const idealRotation = isSlowed
//       ? (this.maxRotation * rotationSize) / this.decelerationTolerance
//       : this.maxRotation;
//
//     const nextIdealRotation = idealRotation * (rotation / rotationSize);
//
//     const angular =
//       (nextIdealRotation - kinematic.rotation) / this.timeToTarget;
//
//     const angularAcceleration = Math.abs(angular);
//     const finalAngular =
//       angularAcceleration > kinematic.maxAngularAcceleration
//         ? (angular * kinematic.maxAngularAcceleration) / angularAcceleration
//         : angular;
//
//     return {
//       angular: finalAngular,
//       linear,
//     };
//   }
// }
//
