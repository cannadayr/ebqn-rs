#!/usr/bin/env bqn

# Modified version of https://github.com/mlochbaum/BQN/blob/master/src/cjs.bqn, which is under the ISC license (https://github.com/mlochbaum/BQN/blob/master/LICENSE)

args←•args
"Usage: ./crs.bqn path/to/mlochbaum/BQN <an expression>"!2≤≠args
return ← 1≡⊑args
args↓˜↩ return
path←(⊑args)∾"/src/"
args↓˜↩1
Import ← {𝕨•Import path∾𝕩}
FChars ← {𝕨•FChars path∾𝕩}
FLines ← {𝕨•FLines path∾𝕩}

# Javascript/JSON formatting
List ← "vec!["∾"]"∾˜(0<≠)◶⟨"",1↓·∾","⊸∾¨⟩ # Native list/array
Tuple ← "("∾")"∾˜(0<≠)◶⟨"",1↓·∾","⊸∾¨⟩ # Native tuple
Block ← Tuple # Native block
Body ← Tuple # Native body
Code ← "Code::new("∾")"∾˜(0<≠)◶⟨"",1↓·∾","⊸∾¨⟩ # Native code
Compressed ← "Bodies::Comp("∾⊢∾")"˙ # compressed bodies definition
Exp ← ≠◶⟨"","Bodies::Head(","Bodies::Exp("⟩∾")"∾˜(0<≠)◶⟨"",1↓·∾","⊸∾¨⟩ # expanded bodies definition
Ind ← {∾𝕨‿"["‿𝕩‿"].clone()"}  # Native list/array indexing
Cat ← {∾𝕨‿".concat("‿𝕩‿")"}   # Native list/array concatenation (like ∾)
# Escape the special characters that appear in BQN sources.
Esc←{
  in ← (@+0‿9‿10‿13)∾"'"""    # Null, Tab, LF, CR, and quotes
  out ← "0tnr"                # Whitespace characters changed to letters
  i ← in⊐𝕩
  𝕩 ↩ i ⊏⟜out⌾((i<≠out)⊸/) 𝕩  # Replace
  ∾(i<≠in) /⟜"\"⊸∾¨ 𝕩         # Insert \
}⍟(0<≠)
Str ← "new_string("""∾Esc∾""")"˜     # A BQN string
Char ← "new_char('"(∾∾⊣○"')")Esc∘⥊   # A BQN character
F ← {'-'¨⌾((𝕩='¯')⊸/)𝕩} •Repr # Native format
FP ← ∞⊸=◶⟨")"∾˜"new_scalar("∾F,"new_scalar(INFINITY)"⟩       # Format positive number
FN ← ¯∞⊸=◶⟨")"∾˜"new_scalar("∾F,"new_scalar(NEG_INFINITY)"⟩       # Format positive number
Num ← 0⊸≤◶⟨FN,FP⟩
FBlock ← Block ⟨F,⊑⟜"false"‿"true",=◶⟨Compressed F,Exp (List F¨)¨⟩⟩ {𝕎𝕩}¨ ⊢

glyphs ← Import "glyphs.bqn"
_getComp ← { (4+useInd) ↑ (𝕗 Import "c.bqn"){𝔽} }
useInd ← "-i"≡⊑args ⋄ args↓˜↩useInd
Comp ← ((<"runtime.r" Ind F)¨↕≠∾glyphs) glyphs _getComp ⊢
J ← ∾∾⟜(@+10)¨
Fconst ← ≡◶⟨@⊸≤◶{Num𝕩}‿Char, Str, ⊑⟩
Fout ← (≠↑⟨F,Fconst,FBlock,Body·F¨2⊸↑,List F⟩˙) {List 𝕎¨𝕩}¨ ⊢
Long ← ∾ (≠↑1‿3/⟨"  "⊸∾⋄((@+10)∾" ,")⊸∾⟩˙) {𝕎𝕩}¨ ⊢
LFC ← Long∘Fout∘Comp

RT ← {
  src‿need‿inputs←𝕩Import"pr.bqn"
  pr←"runtime_0.r"‿"provide.r"{(∾𝕨<⊸(<∘Ind⟜F¨)⟜(↕≠)¨𝕩)⊏˜(∾𝕩)⊐∾need}○((-1+1=𝕩)⊸↑)inputs
  Long Fout pr need _getComp src
}
CArg ← {Num↩π⊸=◶Num‿"new_scalar(std::f64::consts::PI)" ⋄ J (¯5⊸↓∾𝕩˙)⌾⊑ FLines "c.bqn"}
SVG ← {∾⟨"Modify←GetHighlights←⊢⋄"⟩∾ FChars∘∾⟜".bqn"¨ "../svg"‿𝕩}

•Out (⊑"r"‿"r0"‿"r1"‿"c"‿"cc"‿"f"‿"e"‿"p"⊐⊏)◶⟨
  RT∘2, RT∘0, RT∘1
  {𝕩⋄LFC CArg "⟨"∾"⟩"«∾","⊸∾¨'"'(⊣∾∾˜)¨glyphs}
  {𝕩⋄LFC "{"∾"}"∾˜CArg"𝕩"}
  {𝕩⋄LFC FChars "f.bqn"}
  {𝕩⋄LFC SVG "e"}
  {𝕩⋄LFC SVG "p"}
  ¯1 ↓ · J Code∘Fout∘Comp¨
⟩ args
