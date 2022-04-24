# 暗号のアルゴリズム

## 数式

$N$: 素数, $e, k \in \mathbb{F}_N$

* $eG = P$
* $kG = R$
* $uG + vP = kG$
* $u=\dfrac{z}{s}$, $v=\dfrac{r}{s}$

## 署名の作成

$(e, z) \mapsto (r, s)$

1. 秘密鍵 $e \in \mathbb{F}_N$ と署名ハッシュ $z \in {\rm \mathbb{F}_N}$ を取る.
2. 署名ハッシュ $z$ から $k \in \mathbb{F}_N$ がランダムっぽく定まる. (RFC6979)
3. $R:=(r, *):=kG$ から $r \in \mathbb{F}_p$ を求める.

4. $s:=\dfrac{z+re}{k} \in \mathbb{F}_N$ とおく.

## 署名の検証

与えられた $(r, s)$ の検証

1. $(r, s) \in \mathbb{F}_N^2$ と署名ハッシュ $z \in {\rm U256}$, 公開鍵 $P:=eG$ が与えられているとする.

2. $u=\displaystyle \frac{z}{s}$, $v=\displaystyle \frac{r}{s}$ なので
$uG + vP =\displaystyle \frac{z + re}{s}G = kG = R$ である. $R$ の $x$ 座標が $r$ であることをチェックする.
