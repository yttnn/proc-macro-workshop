# memo
## builder
### 01-parse
- 特になにかするテストではないので、空のTokenStreamを返せばOK
  - `lib.rs`に実装
- `syn::DeriveInput`のDOCを見る
### 02-create-builder
- マクロの仕事は、「トークン列を受け取り、必要な変更を加えて、変更後のトークン列を返すこと」
  - トークン列は`TokenStream`,マクロの仕事をこなすのが`derive`関数
- `syn`crateは、`TokenStream`を受け、ASTとして扱えるようにするもの
- `quote`crateは、`syn`の構造から、`TokenStream`にするもの
- dependenciesに加筆
  - `extra-traits` Debug traitを使うために必要だった
- https://github.com/realth000/proc-macro-workshop 参考にさせて頂いた
### 04-call-build
- 値が正しいかチェックするのは、変数がOptionのため、noneかを見れば良さそう
- Box ...?
  - https://doc.rust-jp.rs/rust-by-example-ja/error/multiple_error_types/boxing_errors.html
  - 関数内で複数種類のエラーが発生する可能性があるときに取る手段の一つ
- Rustのreturn時のセミコロン省略は最後のみ、ということを知らず、盛大にハマる
- newで書けないのかな、とも思ったが、公式DOCには「From」を介して、とあるので使わないのかなと思った
- もっときれいに書けそう。他の人の回答も見たい。