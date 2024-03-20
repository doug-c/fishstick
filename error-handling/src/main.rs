use anyhow::Result;

// Define a function named divide that takes two integers and returns a Result.
// This function returns Err if the divisor is zero or one, and Ok otherwise.
fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
  // Check if the divisor is zero.
  if y == 0 {
      // If it is, return an error indicating division by zero.
      Err("Cannot divide by zero")
  } else if y == 1 {
      // If it is, return an error indicating division by zero.
      Err("Cannot divide by one")   
  } else {
      // If not, return the result of the division.
      Ok(x / y)
  }
}

// pass up the error to the caller
fn test_main() -> Result<i32, &'static str> {  
  let result = divide(10, 0)?; // indicates that any error will be passed up to the caller
  Ok(result)
}

// do not pass up the error to the caller; handle it here
fn test_main2() -> Result<(), &'static str> {  
  let result = divide(10, 0); // indicates that any error will be handled with the match statement
  match result {
      Ok(value) => {
          println!("Result: {}", value);
          Ok(())
      },
      Err(err) => {
          println!("Error: {}", err);
          Err(err)
      },
  }
}

// do not pass up the error to the caller; handle it here
fn test_main3() -> Result<(), &'static str> {  
  let result = divide(10, 1); // indicates that any error will be handled with the match statement
  match result {
      Ok(value) => {
          println!("Result: {}", value);
          Ok(())
      },
      Err(err) => {
          println!("Error: {}", err);
          Err(err)
      },
  }
}

// pass up the error to the caller
fn test_main4() -> Result<(), &'static str> {  
    divide(10, 1)?;
    Ok(())
  }

fn test_main5() -> Result<(), &'static str> {  
    divide(10, 2)?;
    Ok(())
}  

fn main() -> Result<()> {
    // Call the divide function with arguments 10 and 2.
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    // Call the divide function with arguments 10 and 0.
    let result2 = divide(10, 0);
    match result2 {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    println!("dac: {:?}", test_main());
    println!("dac2: {:?}", test_main2());
    println!("dac3: {:?}", test_main3());
    println!("dac4: {:?}", test_main4());
    println!("dac5: {:?}", test_main5());

    if test_main5().is_ok() {
        println!("test_main5 is ok");
    } else {
        println!("test_main5 is not ok");
    }

    if test_main5().is_err() {
        println!("test_main5 is err");
    } else {
        println!("test_main5 is not err");
    }

    //let result = divide(10, 1)?; // no where to pass the error to; cannot use the `?` operator in a function that returns `()`
    //println!("Result: {}", result);

    Ok(())

}
