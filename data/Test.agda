module Test where

open import Agda.Builtin.String

data XY : Set where
  a : XY
  b : XY


main : String
main = primStringAppend "bla" "<=> bla bla"



