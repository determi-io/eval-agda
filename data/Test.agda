module Test where

open import Agda.Builtin.String

data XY : Set where
  a : XY
  zy : XY -> XY -> XY
  b : XY


main : XY
main = zy a b

-- primStringAppend "bla" "<=>\n bla bla"



