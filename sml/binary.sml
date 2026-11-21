(* Part II: Binary Addition/Subtraction *)

(* Reverses a list by moving the front item to the back *)
fun reverse [] = []
    (* reverse the rest and put the first item at the end *)
  | reverse (first :: rest) = reverse rest @ [first];

(* turns a list of 1s and 0s into a list of bools *)
fun toBools [] = []
    (* A 0 becomes false *)
  | toBools (0 :: rest) = false :: toBools rest
    (*Anything else is assumed to be a 1 and becomes true *)
  | toBools (_ :: rest) = true :: toBools rest;

(* turns a list of bools back into a list of 1s and 0s*)
fun toInts [] = []
    (* false becomes a 0 *)
  | toInts (false :: rest) = 0 :: toInts rest
    (*true becomes a 1*)
  | toInts (true :: rest) = 1 :: toInts rest;

(*Adds two bits and a carry and gives back the sum bit and the new carry *)
fun addBits (false, false, false) = (false, false)
    (* only one bit is set so 1 *)
  | addBits (true, false, false) = (true, false)
  | addBits (false, true, false) = (true, false)
  | addBits (false, false, true) = (true, false)
    (* two bits are set so 10 *)
  | addBits (true, true, false) = (false, true)
  | addBits (true, false, true) = (false, true)
  | addBits (false, true, true) = (false, true)
    (*all three bits are set so 11 *)
  | addBits (true, true, true) = (true, true);

(* Finishes up when one list runs out before the other*)
fun finishBinaryAdd ([], false) = []
    (*Nothing left but there is still a carry *)
  | finishBinaryAdd ([], true) = [true]
    (* no carry anymore, so the rest of the bits are copied *)
  | finishBinaryAdd (bits, false) = bits
    (* A 0 plus the carry becomes 1 and then stops *)
  | finishBinaryAdd (false :: tail, true) = true :: tail
    (* A 1 plus the carry becomes 0 and the carry continues *)
  | finishBinaryAdd (true :: tail, true) = false :: finishBinaryAdd (tail, true);

(* Does the main adding work bit by bit *)
fun doBinaryAddition ([], [], carry) = finishBinaryAdd ([], carry)
    (* One list ran out so pass the other list and the carry along *)
  | doBinaryAddition ([], rest, carry) = finishBinaryAdd (rest, carry)
  | doBinaryAddition (rest, [], carry) = finishBinaryAdd (rest, carry)
    (*Both lists still have bits so add the two front bits along with the carry *)
  | doBinaryAddition (x :: xs, y :: ys, carry) =
        let
            val (sum, newCarry) = addBits (x, y, carry)
        in
            (* Put the sum bit in front and recurse on the rest of both lists *)
            sum :: doBinaryAddition (xs, ys, newCarry)
        end;

(*adds two lists of 1s and 0s*)
fun binaryAddition (a, b) =
    toInts (reverse (doBinaryAddition (reverse (toBools a), reverse (toBools b), false)));

(*flips every bit so 0s become 1s and 1s become 0s*)
fun flipBits [] = []
    (* Flip the front bit and recurse on the rest *)
  | flipBits (first :: rest) = not first :: flipBits rest;

(* Adds 0s to the end of the first list until it is as long as the second list *)
fun padBits (rest, []) = rest
    (*The first list ran out so add a 0 *)
  | padBits ([], _ :: os) = false :: padBits ([], os)
    (* Both lists still have bits so keep the front bit *)
  | padBits (x :: xs, _ :: os) = x :: padBits (xs, os);

(* keeps only as many bits as the second list has *)
fun keepSameLength ([], _) = []
  | keepSameLength (_, []) = []
    (* If both lists still have bits left *)
  | keepSameLength (x :: xs, _ :: os) = x :: keepSameLength (xs, os);

(* gets rid of extra 0s at the front but always leaves at least one bit *)
fun removeLeadingZeros (0 :: next :: rest) = removeLeadingZeros (next :: rest)
  | removeLeadingZeros bits = bits;

(* Subtracts the second list from the first *)
fun binarySubtraction (a, b) =
    let
        (* Reverse both lists and turn them into bools *)
        val aBits = reverse (toBools a)
        val bBits = reverse (toBools b)
        (* Pad both lists with 0s so they are the same length *)
        val aPadded = padBits (aBits, bBits)
        val bPadded = padBits (bBits, aBits)
        (* Flip every bit of the second list and add 1 with the carry to get the compliment *)
        val sum = doBinaryAddition (aPadded, flipBits bPadded, true)
    in
        (* Drop the extra carry bit at the end *)
        removeLeadingZeros (toInts (reverse (keepSameLength (sum, aPadded))))
    end;

(* Turns a list of 1s and 0s into a string so it is easier to read *)
fun bitsToString [] = ""
    (* Put the front bit in the string *)
  | bitsToString (first :: rest) = Int.toString first ^ bitsToString rest;

(* Prints out an addition problem and its answer *)
fun showAddition (a, b) =
    print (bitsToString a ^ " + " ^ bitsToString b ^ " = " ^ bitsToString (binaryAddition (a, b)) ^ "\n");

(* Prints out a subtraction problem and its answer*)
fun showSubtraction (a, b) =
    print (bitsToString a ^ " - " ^ bitsToString b ^ " = " ^ bitsToString (binarySubtraction (a, b)) ^ "\n");

(* Examples from the project doc to test *)
print "Binary Addition:\n";
showAddition ([1, 1, 1, 1, 0], [1, 0, 1, 1]);
showAddition ([1, 0, 0, 1, 1, 0, 1], [0, 0, 1, 0, 0, 1, 0]);
showAddition ([1, 0, 0, 1, 0, 0, 1], [0, 0, 1, 1, 0, 0, 1]);
showAddition ([1, 0, 0, 0, 1, 1, 1], [1, 0, 1, 1, 0]);

print "Binary Subtraction:\n";
showSubtraction ([1, 0, 1, 0, 0, 1], [1, 0, 1, 1]);
showSubtraction ([1, 0, 1, 1, 1, 1, 1], [0, 0, 1, 0, 0, 1, 0]);
showSubtraction ([1, 1, 0, 0, 0, 1, 0], [0, 0, 1, 1, 0, 0, 1]);
showSubtraction ([1, 0, 1, 1, 1, 0, 1], [1, 0, 1, 1, 0]);
showSubtraction ([1, 0, 1, 0], [1, 0, 1, 0]);
