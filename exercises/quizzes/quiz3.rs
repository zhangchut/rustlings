// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

// TODO: Adjust the struct as described above.
enum Grade {
    Float(f32),
    Str(&'static str)
}
#[derive(Debug)]
enum IntoGradeError {
    FloatOutOfRnage,
    StrNotGrade
}
impl TryFrom<f32> for Grade {
    type Error = IntoGradeError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if let 1.0..=5.5 = value {
            Ok(Grade::Float(value))
        } else {
            Err(IntoGradeError::FloatOutOfRnage)
        }
    }
}
impl TryFrom<&'static str> for Grade {
    type Error = IntoGradeError;
    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() == 2 && (b'A'..=b'F').contains(&bytes[0]) && matches!(bytes[1], b'+' | b'-') {
            Ok(Grade::Str(&value))
        } else {
            Err(IntoGradeError::StrNotGrade)
        }
    }
}
impl core::fmt::Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grade::Float(g) => write!(f, "{}", g),
            Grade::Str(g) => write!(f, "{}", g)
        }
    }
}
struct ReportCard {
    grade: Grade,
    student_name: String,
    student_age: u8,
}

// TODO: Adjust the impl block as described above.
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1.try_into().unwrap(),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+".try_into().unwrap(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
