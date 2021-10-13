TypeScript 3.8
====

## Type-Only Imports and Exports

- ほとんどのユーザには考える必要がない機能ではあるが，TypeScript APIまたは Babelで利用されている可能性がある．

```ts
import type { SomeThing } from "./some-module.js"

export type { SomeThing }

```


## EcmaScript Private Field

ECMA Script で，private fieldsのproposalがstage3になったのでサポートとなる

private filedは，正規のプロパティと異なり，いくつかの付随するルールがある

- private fieldsは， 単一の *#* 文字からはじまる

- すべての private fieldsは，それを含むクラスで一意のスコープを持つ

- TypeScriptの*public* *private* アクセス修飾子は，private fieldsには使えない

- Private fieldsは，外側のクラスからアクセスしたり検出することは出来ない．


## Top Level await

TypeScirpt 3.8からECMAScriptで，"top-level await"と呼ばれている機能のサポートが入る．

モジュールのTopLevelで*await*を使うことができるようになる

Top Level awaitはモジュールのトップレベルで動作する．これは，TypeScriptがimport / exportを発見したときにのみモジュールとに見なされるため， これを確実にするために `export {}`などの定型文を書く必要があるかもしれない．

環境によっては動作しない可能性がある．ターゲットコンパイラオプションがES2017以降でモジュールがESNextまたはsystemでの場合のみのサポート


## JSDoc Property Modifiers
JSDocにプロパティ修飾子のサポートが追加される．javascriptの方サポートはこれまで，allowJSオプションを有効化し，checkJSオプションでファイルの型チェックを行っていたが，ファイルの先頭に *@ts-check* と書くことでも型チェックがサポートされるようになる


- @public

- @protected

- @private

- @readonly

## “Fast and Loose” Incremental Checking

新しいコンパイラオプション *assumeChangesOnlyAffectDirectDependencie*が導入される．このオプションは影響を受けるファイル以外の再チェックや再構築を回避することが出来るようになる．