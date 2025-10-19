use crate::UnitExponents;

impl UnitExponents {
    /**
    Returns the [`UnitExponents`] for time.
     */
    pub const fn time() -> Self {
        return Self {
            second: 1,
            meter: 0,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for length.
     */
    pub const fn length() -> Self {
        return Self {
            second: 0,
            meter: 1,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for mass.
     */
    pub const fn mass() -> Self {
        return Self {
            second: 0,
            meter: 0,
            kilogram: 1,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for (electrical) current.
     */
    pub const fn electrical_current() -> Self {
        return Self {
            second: 0,
            meter: 0,
            kilogram: 0,
            ampere: 1,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for temperature.
     */
    pub const fn temperature() -> Self {
        return Self {
            second: 0,
            meter: 0,
            kilogram: 0,
            ampere: 0,
            kelvin: 1,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for amount of substance.
     */
    pub const fn amount_of_substance() -> Self {
        return Self {
            second: 0,
            meter: 0,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 1,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for luminous intensity.
     */
    pub const fn luminous_intensity() -> Self {
        return Self {
            second: 0,
            meter: 0,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 1,
        };
    }

    /**
    Returns the [`UnitExponents`] for surface area.
     */
    pub const fn area() -> Self {
        return Self {
            second: 0,
            meter: 2,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for volume.
     */
    pub const fn volume() -> Self {
        return Self {
            second: 0,
            meter: 3,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for (electrical) voltage.
     */
    pub const fn electrical_voltage() -> Self {
        return Self {
            second: -3,
            meter: 2,
            kilogram: 1,
            ampere: -1,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for force.
     */
    pub const fn force() -> Self {
        return Self {
            second: -2,
            meter: 1,
            kilogram: 1,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for torque.
     */
    pub const fn torque() -> Self {
        return Self {
            second: -2,
            meter: 2,
            kilogram: 1,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for power.
     */
    pub const fn power() -> Self {
        return Self {
            second: -3,
            meter: 2,
            kilogram: 1,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for energy.
     */
    pub const fn energy() -> Self {
        return Self {
            second: -2,
            meter: 2,
            kilogram: 1,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for frequency.
     */
    pub const fn frequency() -> Self {
        return Self {
            second: -1,
            meter: 0,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for (linear) velocity.
     */
    pub const fn velocity() -> Self {
        return Self {
            second: -1,
            meter: 1,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for (angular) velocity.
     */
    pub const fn angular_velocity() -> Self {
        return Self {
            second: -1,
            meter: 0,
            kilogram: 0,
            ampere: 0,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for magnetic flux.
     */
    pub const fn magnetic_flux() -> Self {
        return Self {
            second: -2,
            meter: 2,
            kilogram: 1,
            ampere: -1,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for magnetic flux density.
     */
    pub const fn magnetic_flux_density() -> Self {
        return Self {
            second: -2,
            meter: 0,
            kilogram: 1,
            ampere: -1,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for inductance.
     */
    pub const fn inductance() -> Self {
        return Self {
            second: -2,
            meter: 2,
            kilogram: 1,
            ampere: -2,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for (electrical) conductance.
     */
    pub const fn electrical_conductance() -> Self {
        return Self {
            second: 3,
            meter: -2,
            kilogram: -1,
            ampere: 2,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for (electrical) resistance.
     */
    pub const fn electrical_resistance() -> Self {
        return Self {
            second: -3,
            meter: 2,
            kilogram: 1,
            ampere: -2,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for (electrical) conductivity.
     */
    pub const fn electrical_conductivity() -> Self {
        return Self {
            second: 3,
            meter: -3,
            kilogram: -1,
            ampere: 2,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }

    /**
    Returns the [`UnitExponents`] for (electrical) resistivity.
     */
    pub const fn electrical_resistivity() -> Self {
        return Self {
            second: -3,
            meter: 3,
            kilogram: 1,
            ampere: -2,
            kelvin: 0,
            mol: 0,
            candela: 0,
        };
    }
}
