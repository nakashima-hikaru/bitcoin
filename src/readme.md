# 暗号のアルゴリズム

## 署名の作成

1. 秘密鍵 $e \in \mathbb{F}_N$ と署名ハッシュ $z \in {\rm U256}$ を取る.
2. 署名ハッシュ $z$ から $k \in \mathbb{F}_N$ が定まる.
3. $R:=(r, *):=kG$ から $r \in \mathbb{F}_p$ を求める.

4. $s:=\displaystyle \frac{z+re}{k} \in \mathbb{F}_N$ とおく. 
5. ここから, $u:=\displaystyle \frac{z}{s} \in \mathbb{F}_N$, $v:=\displaystyle \frac{r}{s} \in \mathbb{F}_N$ と定める.

## 署名の検証

1. $(r, s) \in \mathbb{F}_N^2$ と署名ハッシュ $z \in {\rm U256}$, 公開鍵 $P:=eG$ が与えられているとする.

2. $u=\displaystyle \frac{z}{s}$, $v=\displaystyle \frac{r}{s}$ なので
$uG + vP =\displaystyle \frac{z + re}{s}G = kG = R$ である. $R$ の $x$ 座標が $r$ であることをチェックする.
