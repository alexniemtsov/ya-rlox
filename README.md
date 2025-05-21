yet Another Rust Lox compiler

todo: Workflow:
Language specs: 
  Dynamic types: bool, num (double-precigion floating point also prepresenting Int), array, string (array of chars, declared with double quites), object, nil
  Expressions (produce a Value): 
      Arithmetic: Binary operators, (in/pre/post)fix and with with numbers, except "+" that can be used to concat str
      Comparison + Equality: a < b, a > b, a <= b, a >= b, a == b, a != b
      Logical: !true, !false, true and false, true or false. So `and` check is Both are true (if
        left = false then right isn't validated). Same with `or`, if left operand is true then right\
        is ignored.
      Bitwise operations: declared by keywords `AND, OR, NOR, XOR`

      Precedence and groupping: Round brackets `()`

  Statements (produce Effect: modify state, read input, produce output) e.g. `print`
      Semicolon promotes expression to statement

  Scoping: `{}` curly brackets (packs a series of statements is a single one)

  Variables: declared with `var` statement, if initializer is ommited the value defaults to
  `nil`. Once declared it can be accessed and mutated with-in the scope.

  Control Flow:
      Conditional: `if`, `else`, `else-if`
      Loops: `while`, `for`

  Functions: `my_func(param1, param2)` or `my_func()`, declaring functions with params:
      ```
      fun print_sum(a, b) {
          print a + b;
      }
      ```
      Arguments are passed into the functions, parameters (formals) are declared inside it.

      Body of the function is always a block `{}` and optionally can `return value`.
      If exectuion reaches the end of the block without hitting return statement - `nil` wil be
      returned

  Closures: functions that keep references to any variables used inside so they stay when
  parent goes out of scope


  OOP:
      Classes. Has methods and properties. Props are dynamic, can be assigned during runtime
      only.

      Instance can be created by calling classname as a function:
          ```
          class Greeting {
              hello() {
                  print "Hello";
              }
              
              world(x) {
                  print "World " + x; concat
              }
          }

          var class_ref = Greeting; Store reference to a class in variable
          some_func(Greeting); Pass reference to a class into func

          var instance = Greeting();  Create new instance and store it into variable
          print instance;  "Greeting instance"
          instance.hello();  "Hello"
          instance.world(100);  "World 100"
          ```
  
      Props:
          ```
          class MyClass {
              get_msg() {
                  return this.msg;
              }

              set_msg(v) {
                  this.msg = v;
              }
          }
          
          var val = MyClass();
          print val.msg;  nil
          val.msg = "hello";
          print val.msg == val.get_msg();  true
          print val.set_msg("world");
          print val.msg == val.get_msg();  true
          print val.msg == "hello";  false

          print val.new_prop;  nil
          val.new_prop = "foo";
          print val.new_prop;  "foo"

          val.second_prop = val.new_prop;
          print val.second_prop;  "foo"


      Constructor: declared with method called `init(...)`

          class Breakfast {
              init(meat, bread) {
                  this.meat = meat;
                  this.bread = bread;
              }
           ...
          }
          var baconAndToast = Breakfast("bacon", "toast");
          baconAndToast.serve("Dear Reader");
           "Enjoy your bacon and toast, Dear Reader."
      
      Inheritance:
          Declared using `<` operator.
          ```
          class Brunch < Breakfast {
               ...
          }
          ```

          Every method defined in superclass available to its subclasses
          Parent class can always be accessed using `super` keyword


          


  STD:

  `print`
  `clock()`



  tracing garbage collection

Front-end: clean (remove comments, spaces) -> tokenize -> create AST (abstract syntax tree) -> optimize (static analysis)
Backend: generator, to bytecode
