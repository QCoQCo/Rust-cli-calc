use regex::Regex;

pub fn evl_ex(expression:&str)->Result<f64,&'static str>{
    let re=Regex::new(r"(\d+\.?\d*)\s*([\+\-\*/])\s*(\d+\.?\d*)").unwrap();
    
    if let Some(caps)=re.captures(expression){
        let lefty:f64=caps.get(1).unwrap().as_str().parse().map_err(|_| "Invalid number")?;
        let opr=caps.get(2).unwrap().as_str();
        let righty:f64=caps.get(3).unwrap().as_str().parse().map_err(|_| "Invalid number")?;

        let result=match opr{
            "+"=>lefty + righty,
            "-"=>lefty - righty,
            "*"=>lefty * righty,
            "/"=>{
                if righty == 0.0{
                    return Err("Cannot divide by ZERO");
                }
                lefty / righty
            },
            _=>return Err("Unknown operator"),
        };
        Ok(result)
    }else{
        Err("Invalid expression")
    }
}