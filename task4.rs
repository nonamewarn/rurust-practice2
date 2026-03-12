fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&g| {
            if g < 38 {
                g
            } else {
                let remainder = g % 5;
                if 5 - remainder < 3 {
                    g + (5 - remainder)
                } else {
                    g
                }
            }
        })
        .collect()
}

