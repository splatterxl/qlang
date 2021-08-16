# qlang
idk

an example of qlang is
```
struct User ::
  prop name :~ String ::
    default ""
  ;;
;;

impl User ::
  declare function aaaa ::
    params [ (thing : String) (num : Int) ];
    body ::
      debug ((thing))
    ;;
  ;;

  # aaaa
;;
```
