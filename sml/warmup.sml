(* Part I: SML & Rust Warmup *)

(* Checks if 42 shows up in the list *)
fun hitchhiker [] = false
    (* The front number is 42 so we found it *)
  | hitchhiker (42 :: _) = true
    (* not 42 so keep checking the rest *)
  | hitchhiker (_ :: rest) = hitchhiker rest;

(*Keeps every other number from the list *)
fun everyother [] = []
    (* only one number left so it gets skipped*)
  | everyother [_] = []
    (* Skip the first number and keep the second and recurse on the rest *)
  | everyother (_ :: second :: rest) = second :: everyother rest;

(* Checks that nothing from d up to the square root of n divides n *)
fun noDivisors (n, d) =
    case (d * d > n, n mod d = 0) of
        (*Went past the square root so there are no divisors *)
        (true, _) => true
        (* d divides n evenly so it is not prime *)
      | (false, true) => false
        (* d doesn't divide n so try the next one*)
      | (false, false) => noDivisors (n, d + 1);

(* checks if a number is prime and anything less than 2 is not prime *)
fun isPrime n = n >= 2 andalso noDivisors (n, 2);

(* Checks each number starting at p until they are both prime *)
fun findPrimes (n, p) =
    case (isPrime p, isPrime (n - p)) of
        (*Both are prime so print the answer *)
        (true, true) => print (Int.toString p ^ " + " ^ Int.toString (n - p) ^ " = " ^ Int.toString n ^ "\n")
        (*At least one is not prime so try the next number*)
      | _ => findPrimes (n, p + 1);

(* Finds two primes that add up to the number and prints them *)
fun goldbach n =
    case (n > 2, n mod 2 = 0) of
        (* number is not bigger than 2 so it doesn't work *)
        (false, _) => print (Int.toString n ^ " has to be greater than 2\n")
        (* number is odd so it doesn't work *)
      | (_, false) => print (Int.toString n ^ " has to be an even number\n")
        (* Number is good so start looking for primes at 2 *)
      | (true, true) => findPrimes (n, 2);

(* examples to test hitchhiker *)
hitchhiker [1, 2, 42, 4];
hitchhiker [1, 2, 3, 4];

(*example from the project doc to test everyother*)
everyother [3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43];

(* examples to test goldbach and its error checking*)
goldbach 28;
goldbach 4;
goldbach 100;
goldbach 15;
goldbach 2;
