//! Data structures that represent the information contained in the complex coefficient of a ket.

#[derive(Copy, Clone)]
pub struct FloatCoefficient {
    magnitude: f64,
    imaginary: bool
}

/// Initializes a float coefficient.
pub fn create_coefficient(magnitude:f64, imaginary:bool) -> FloatCoefficient {
    FloatCoefficient{magnitude: magnitude, imaginary: imaginary}
}

pub trait Coefficient {
    type StructType;

    fn equals_coefficient(&self, other: Self::StructType) -> bool;
    fn multiply_by_coefficient(&self, other: Self::StructType) -> Self::StructType;
    fn add_to_coefficient(&self, other: Self::StructType) -> Self::StructType;
    fn get_magnitude(&self) -> f64;
    fn get_imaginary(&self) -> bool;
    fn set_magnitude(&mut self, magnitude:f64);
    fn set_imaginary(&mut self);
    fn clear_imaginary(&mut self);
    fn negate_magnitude(&mut self);
    fn multiply_by_i(&mut self);
    fn multiply_by_number(&mut self, number:f64);
    fn to_probability(&self) -> f64;
    fn complex_conjugate(&mut self) -> Self::StructType;
    fn print(&self);
}

impl Coefficient for FloatCoefficient {
    type StructType = FloatCoefficient;

    /// Checks whether the coefficient is equal to another which is purely real or purely imaginary.
    fn equals_coefficient(&self, other: Self::StructType) -> bool {
        (self.magnitude == other.get_magnitude()) && (self.imaginary == other.get_imaginary())
    }

    /// Multiplies the coefficient by another which is purely real or purely imaginary.
    fn multiply_by_coefficient(&self, other: Self::StructType) -> Self::StructType {
        let new_magnitude:f64 = self.magnitude*other.get_magnitude();
        let mut new_coeff= create_coefficient(new_magnitude, false);
        if self.imaginary == true && other.imaginary == true {
            new_coeff.negate_magnitude();
        }
        else if self.imaginary == true || other.imaginary == true {
            new_coeff.set_imaginary();
        }
        new_coeff
    }

    /// Adds the coefficient to another which is purely real or purely imaginary.
    fn add_to_coefficient(&self, other: Self::StructType) -> Self::StructType {
        if !(other.get_imaginary() == self.imaginary) {
            panic!("attempt to add imaginary to real coefficient using wrong method.");
        }
        let new_imaginary = self.imaginary;
        let new_magnitude = self.magnitude + other.get_magnitude();
        let result = create_coefficient(new_magnitude, new_imaginary);
        result
    }

    /// Gets the magnitude of the coefficient.
    fn get_magnitude(&self) -> f64 {
        self.magnitude
    }

    /// Whether the coefficient is imaginary or real.
    fn get_imaginary(&self) -> bool {
        self.imaginary
    }

    /// Sets the magnitude of the coefficient.
    fn set_magnitude(&mut self, magnitude:f64) {
        self.magnitude = magnitude;
    }

    /// Makes this coefficient imaginary.
    fn set_imaginary(&mut self) {
        self.imaginary = true;
    }

    /// Makes this coefficient real.
    fn clear_imaginary(&mut self) {
        self.imaginary = false;
    }

    /// Negates the coefficient.
    fn negate_magnitude(&mut self) {
        self.set_magnitude(-self.magnitude);
    }

    /// Multiplies the coefficient by i.
    fn multiply_by_i(&mut self) {
        if self.imaginary {
            self.negate_magnitude();
            self.clear_imaginary();
        }
        else if !(self.imaginary == true) {
            self.set_imaginary();
        }
    }

    /// Multiplies the coefficient by a number.
    fn multiply_by_number(&mut self, number:f64) {
        self.magnitude = self.magnitude*number;
    }

    /// Determines the probabilistic weight of the coefficient.
    fn to_probability(&self) -> f64 {
        self.magnitude*self.magnitude
    }

    /// Takes the complex conjugate of the coefficient in place.
    fn complex_conjugate(&mut self) -> Self::StructType {
        if self.imaginary == true {
            self.negate_magnitude();
        }
        *self
    }

    /// Prints the coefficient.
    fn print(&self) {
        let sign:char = if self.magnitude < 0.0 { '-' } else { '+' };
        print!("{}", sign);
        if self.imaginary {
            print!(" i");
        }
        print!(" {:.3} ", self.magnitude);
    }
}

#[derive(Copy, Clone)]
pub struct ComplexCoefficient {
    real_component: FloatCoefficient,
    imaginary_component: FloatCoefficient
}

/// Initializes a complex coefficient with optional real and imaginary components.
pub fn create_complex_coefficient(real_component: FloatCoefficient, imaginary_component: FloatCoefficient) -> ComplexCoefficient {
    ComplexCoefficient{real_component: real_component, imaginary_component: imaginary_component}
}

impl ComplexCoefficient {

    /// Checks whether the coefficient is equal to another which has both real and imaginary components.
    pub fn equals_complex_coefficient(&self, other: ComplexCoefficient) -> bool {
        (self.real_component.equals_coefficient(other.get_real_component()) && self.imaginary_component.equals_coefficient(other.get_imaginary_component()))
    }

    /// Checks whether the coefficient is equal to another which is purely real or purely imaginary.
    pub fn equals_coefficient(&self, other:  FloatCoefficient) -> bool {
        (self.real_component.equals_coefficient(other) || self.imaginary_component.equals_coefficient(other))
    }

    /// Multiplies the coefficient by another which is purely real or purely imaginary.
    pub fn multiply_by_coefficient(&self, other: FloatCoefficient) -> ComplexCoefficient {
        let coeff = other.clone();
        self.multiply_by_coefficient(coeff)
    }

    /// Multiplies the coefficient by another which has both real and imaginary components.
    pub fn multiply_by_complex_coefficient(&self, other: ComplexCoefficient) -> ComplexCoefficient {
        let new_imaginary_component = other.get_real_component().multiply_by_coefficient(self.imaginary_component).add_to_coefficient(other.get_imaginary_component().multiply_by_coefficient(self.real_component));
        let new_real_component = other.get_real_component().multiply_by_coefficient(self.real_component).add_to_coefficient(other.get_imaginary_component().multiply_by_coefficient(self.imaginary_component).complex_conjugate());
        create_complex_coefficient(new_real_component, new_imaginary_component)
    }

    /// Adds the coefficient to another which is purely real or purely imaginary.
    pub fn add_to_coefficient(&self, other:  FloatCoefficient) -> ComplexCoefficient {
        if other.get_imaginary() == true {
            let new_imaginary_component = other.add_to_coefficient(self.imaginary_component);
            create_complex_coefficient(self.real_component, new_imaginary_component)
        }
        else {
            let new_real_component = other.add_to_coefficient(self.real_component);
            create_complex_coefficient(new_real_component, self.imaginary_component)
        }
    }

    /// Adds the coefficient to another which has both real and imaginary components.
    pub fn add_to_complex_coefficient(&self, other: ComplexCoefficient) -> ComplexCoefficient {
        let new_imaginary_component = other.get_imaginary_component().add_to_coefficient(self.imaginary_component);
        let new_real_component = self.real_component.add_to_coefficient(other.get_real_component());
        create_complex_coefficient(new_real_component, new_imaginary_component)
    }

    /// Gets the complex coefficient's real component.
    pub fn get_real_component(&self) ->  FloatCoefficient {
        self.real_component
    }

    /// Gets the complex coefficient's imaginary component.    
    pub fn get_imaginary_component(&self) ->  FloatCoefficient {
        self.imaginary_component
    }

    /// Sets the real component of the complex coefficient.            
    pub fn set_real_component(&mut self, real_component:  FloatCoefficient){
        if real_component.get_imaginary() == false {
            self.real_component = real_component;
        }
        else {
            panic!("setting real component to value of incorrect type was attempted");
        }
    }

    /// Sets the imaginary component of the complex coefficient.            
    pub fn set_imaginary_component(&mut self, imaginary_component:  FloatCoefficient){
        if imaginary_component.get_imaginary() == true {
            self.imaginary_component = imaginary_component;
        }
        else {
            panic!("setting imaginary component to value of incorrect type was attempted");
        }
    }

    /// Negates the coefficient.    
    pub fn negate_magnitude(&mut self) {
        self.real_component.negate_magnitude();
        self.imaginary_component.negate_magnitude();
    }

    /// Multiplies the coefficient by i.    
    pub fn multiply_by_i(&mut self) {
        let mut new_real_component = self.imaginary_component.clone();
        new_real_component.clear_imaginary();
        new_real_component.negate_magnitude();
        let mut new_imaginary_component = self.real_component.clone();
        new_imaginary_component.set_imaginary();
        self.real_component = new_real_component;
        self.imaginary_component = new_imaginary_component;
    }

    /// Multiplies the coefficient by a number.
    pub fn multiply_by_number(&mut self, number:f64) {
        self.real_component.multiply_by_number(number);
        self.imaginary_component.multiply_by_number(number);
    }

    /// Determines the probabilistic weight of the coefficient.            
    pub fn to_probability(&self) -> f64 {
        let real_component = self.real_component.get_magnitude();
        let imaginary_component = self.imaginary_component.get_magnitude();
        real_component*real_component + imaginary_component*imaginary_component
    }

    /// Takes the complex conjugate of the coefficient in place.    
    pub fn complex_conjugate(&mut self) {
        self.imaginary_component.negate_magnitude();
    }

    /// Prints the complex coefficient.
    pub fn print(&self) {
        print!(" + (");
        self.real_component.print();
        self.imaginary_component.print();
        print!(" )");
    }
}