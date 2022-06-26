# LakeNetPackage

## What's this?
A library for storing data with a segmented format in a utf-8 string. <br>

## Concept 
Data gets stored in segments, each segments contains a key, and a value (*`key=value`*). Values can be of many types (*[#Supported-types-of-values](See this section for more details)*), including `Null` values, where the key doesn't have any value (*`key`*).
## Syntax example
```
key=string:bool_key=false:int_key=3:list_key=[element1;element2;element3]:key_with_null_value:
```

## Supported types of values
The currently supported 
- `String`
- `Bool`
- `Int` (*i128*)
- `List` (*Vec&lt;String&gt;*)
- `Null` (*The key doesn't point to any value*)
