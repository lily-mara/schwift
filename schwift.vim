if exists("b:current_syntax")
    finish
endif

syntax keyword schwiftKeyword squanch
syntax match schwiftKeyword "show me what you got"
syntax match schwiftKeyword "portal gun"
syntax match schwiftKeyword "on a cob"
syntax keyword schwiftKeyword assimilate
highlight link schwiftKeyword Keyword

syntax keyword schwiftConditional if else
highlight link schwiftConditional Conditional

syntax keyword schwiftLoop while
highlight link schwiftLoop Repeat

syntax match schwiftOperator "\v\*"
syntax match schwiftOperator "\v/"
syntax match schwiftOperator "\v\+"
syntax match schwiftOperator "\v-"
syntax match schwiftOperator "\v\=\="
syntax match schwiftOperator "\v\!"
syntax keyword schwiftOperator less more lessquanch moresquanch

highlight link schwiftOperator Operator

syntax region schwiftString start=/\v"/ end=/\v"/
highlight link schwiftString String

syntax match schwiftInt "\v\d+"
highlight link schwiftInt Number

syntax match schwiftFloat "\v\d+\.\d+"
highlight link schwiftFloat Float

syntax keyword schwiftBool rick morty
highlight link schwiftBool Boolean

let b:current_syntax = "schwift"
