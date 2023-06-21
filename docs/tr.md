# Dizeyler (Matrisler)

## Dizey Tanımı
$a$ satır ve $b$ sütundan oluşan $a*b$ elemanlı tabloya dizey denir ve $M_{a,b}$ şeklinde gösterilir. Dizeyin $i$. satır, $j$. sütununda bulunan eleman $e_{i,j}$ olarak gösterilir.
$$M_{2,3} = \begin{bmatrix}1 & 2 & 3 \\ 4 & 5 & 6\end{bmatrix}$$
$$e_{2,1} = 4$$

### Dizeylerin Eşliği
Eğer $M$ ve $N$ dizeylerinin satır ve sütun sayıları ile elemanları aynı ise $M=N$ denir.

## Dizeylerde İşlemler
### Genişletilmiş Toplama
Satır ve sütun sayıları aynı olan $M$ ve $N$ dizeylerinin toplamı $T = M + N$ olarak gösterilir ve $t_{i,j}=m_{i,j} + n_{i,j}$'dir.

$M_{2,2}=\begin{bmatrix}1 & 2 \\ 3 & 4\end{bmatrix} \qquad$ 
$N_{2,2}=\begin{bmatrix}5 & 6 \\ 7 & 8\end{bmatrix} \qquad$
$M+N=T_{2,2}=\begin{bmatrix}6 & 8 \\ 10 & 12\end{bmatrix}$

### Doğrudan Toplama
$a$ satır ve $b$ sütundan oluşan $M_{a,b}$ dizeyi ile $c$ satır ve $d$ sütundan oluşan $N_{c,d}$ dizeyinin doğrudan toplamı $T = M \oplus N$ olarak gösterilir. T $a+c$ satırlı ve $b+d$ sütunlu bir dizeydir ve örnekteki gibi tanımlanır:

$M_{2,2}=\begin{bmatrix}1 & 2 \\ 3 & 4\end{bmatrix} \qquad$
$N_{2,3}=\begin{bmatrix}5 & 6 & 7 \\ 8 & 9 & 10\end{bmatrix} \qquad$
$M_{2,2} \oplus N_{2,3} = T_{4, 5} = \begin{bmatrix}1 & 2 & 0 & 0 & 0 \\ 3 & 4 & 0 & 0 & 0 \\ 0 & 0 & 5 & 6 & 7 \\ 0 & 0 & 8 & 9 & 10\end{bmatrix}$

### Çıkarma
Satır ve sütun sayıları aynı olan $M$ ve $N$ dizeylerinin farkı $F = M - N$ olarak gösterilir ve $f_{i,j}=m_{i,j} - n_{i,j}$'dir.

$M_{2,2}=\begin{bmatrix}5 & 6 \\ 7 & 8\end{bmatrix} \qquad$
$N_{2,2}=\begin{bmatrix}4 & 3 \\ 2 & 1\end{bmatrix} \qquad$ 
$M+(-1)\cdot N=F_{2,2}=\begin{bmatrix}1 & 3 \\ 5 & 7\end{bmatrix}$

### Skaler ile Çarpma
Bir $M_{a,b}$ dizeyi ile bir $x$ skalerinin çarpımı $C_{a,b} = x \cdot M_{a,b}$ olarak gösterilir ve $c_{i,j} = x*m_{i,j}$'dir.

$x = 5 \qquad$
$M_{2,2}=\begin{bmatrix}1 & 2 \\ 3 & 4\end{bmatrix} \qquad$ 
$x \cdot M=C=\begin{bmatrix}5 & 10 \\ 15 & 20\end{bmatrix}$

### Nokta Çarpımı
$M_{a,b}$ ve $N_{a,b}$ dizeylerinin nokta çarpımı $d = M \cdot N$ olarak gösterilir ve $\sum^a_{i=1} \sum^b_{j=1} m_{i,j}*n_{i,j}$ olarak tanımlanır.

### Dizey Çarpımı
$a$ satır ve $b$ sütundan oluşan bir $M_{a,b}$ dizeyi ile $b$ satır ve $c$ sütundan oluşan $N_{b,c}$ dizeyinin çaprımı $C_{a,c} = M_{a,b} \times N_{b,c}$ olarak gösterilir ve $c_{i,j} = \sum^n_{k=1} m_{i,k}*n_{k,j}$ formülü ile bulunur.

$$\begin{bmatrix} m_{1,1} & m_{1,2} & \cdots & m_{1,b}\\ m_{2,1} & m_{2,2} & \cdots & m_{2,b}\\ \vdots & \vdots & \ddots & \vdots\\ m_{a,1} & m_{a,2} & \cdots & m_{a,b} \end{bmatrix}
\times
\begin{bmatrix} n_{1,1} & n_{1,2} & \cdots & n_{1,c}\\ n_{2,1} & n_{2,2} & \cdots & n_{2,c}\\ \vdots & \vdots & \ddots & \vdots\\ n_{b,1} & n_{b,2} & \cdots & n_{b,c}\end{bmatrix}
=
\begin{bmatrix} c_{1,1} & c_{12} & \cdots & c_{1,c}\\ c_{2,1} & c_{22} & \cdots & c_{2,c}\\ \vdots & \vdots & \ddots & \vdots\\ c_{a,1} & c_{a,2} & \cdots & c_{a,c}\end{bmatrix}$$

Genelde $M\times N \neq N\times M$ olarak kabul edilebilir fakat bu durumu sağlamayan örnekler vardır.

### Tersçapraz (Dizeyin Devriği)
$a$ satır ve $b$ sütunlu bir $M$ dizeyinin tersçaprazı elemanların asal köşegene göre yansıması alınarak (satırlar sütun, sütunlar satır olacak şekilde yeni bir dizey tanımlayarak) elde edilir ve $M^T$ olarak gösterilir, sonuçta $m_{i,j}=m^T_{j,i}$ sağlanır.

## Özel Dizeyler
### Sıfır Dizeyi
Tüm elemanları $0$ olan $a$ satır ve $b$ sütuna sahip dizeye sıfır dizeyi denir ve $0_{a,b}$ şeklinde gösterilir.
$$0_{2, 2} = \begin{bmatrix}0 & 0 \\ 0 & 0\end{bmatrix}$$

### Birim Dizey
Asal köşegeni üzerindeki tüm elemanları $1$, geri kalan tüm elemanları $0$ olan $a$ satır ve $a$ sütundan oluşan dizeylere birim dizey denir.
$$I_{3,3} = \begin{bmatrix}1 & 0 & 0 \\ 0 & 1 & 0 \\ 0 & 0 & 1\end{bmatrix}$$
$M_{a,b} \times I_{b,b} = I_{a,a} \times M_{a,b} = M_{a,b}$'dir.

### Kare Dizey
$a$ satır ve $b$ sütundan oluşan bir dizeye $a=b$ ise kare dizey denir.
$$M_{3,3} = \begin{bmatrix}6 & 22 & 16 \\ 1 & 0 & 12 \\ 23 & 12 & 15\end{bmatrix}$$

### Köşegen Dizey
$a$ satır ve $b$ sütundan oluşan genellikle $a=b$ olan dizeylerde asal köşegeni dışındaki elemanları $0$ olan dizeye köşegen dizey denir.
$$M_{3,3} = \begin{bmatrix}5 & 0 & 0 \\ 0 & 6 & 0 \\ 0 & 0 & 8\end{bmatrix}$$

### Karşıt Köşegen Dizey
$a$ satır ve $b$ sütundan oluşan genellikle $a=b$ olan dizeylerde yedek köşegeni dışındaki elemanları $0$ olan dizeye karşıt köşegen dizey denir.
$$M_{3,3} = \begin{bmatrix}0 & 0 & 1 \\ 0 & 2 & 0 \\ 3 & 0 & 0\end{bmatrix}$$

### Üç Köşegenli Dizey
$a$ satır ve $b$ sütundan oluşan genellikle $a=b$ olan dizeylerde asal köşegeni ve asal köşegene komşu köşegenler dışındaki elemanları $0$ olan dizeye karşıt köşegen dizey denir.
$$M_{5,5} = \begin{bmatrix}1 & 2 & 0 & 0 & 0 \\ 3 & 0 & 2 & 0 & 0 \\ 0 & 3 & 5 & 0 & 0\\ 0 & 0 & 2 & 9 & 4 \\ 0 & 0 & 0 & 1 & 6\end{bmatrix}$$

### Üçgensel Dizey
$a$ satır ve $a$ sütundan oluşan bir dizeyde $i<j$ olan tüm elemanları için $e_{i,j}=0$ şartı sağlanıyorsa bu dizeye **üst üçgensel dizey**, $j>i$ olan tüm elemanları için $e_{i,j}=0$ şartı sağlanıyorsa bu dizeye **alt üçgensel dizey** denir.

Üst üçgensel dizey: $M_{3,3} = \begin{bmatrix}1 & 2 & 3 \\ 0 & 5 & 6 \\ 0 & 0 & 9\end{bmatrix}$, 
alt üçgensel dizey: $N_{3,3} = \begin{bmatrix}1 & 0 & 0 \\ 4 & 5 & 0 \\ 7 & 8 & 9\end{bmatrix}$.

### Simetrik Dizey
$a$ satır ve $a$ sütundan oluşan bir dizeyde $e_{i,j}=e_{j,i}$ ise yani asal köşegene göre simetrik olan elemanlar aynı ise bu dizeye simetrik dizey denir. Bir dizey simetrik dizey ise $M^T=M$ sağlanır.
$$M_{3,3} = \begin{bmatrix}1 & 2 & 3 \\ 2 & 5 & 6 \\ 3 & 6 & 9\end{bmatrix}$$

### Ters Simetrik Dizey
$a$ satır ve $a$ sütundan oluşan bir dizeyde $e_{i,j}=e_{a+1-j,a+1-i}$ ise yani yedek köşegene göre simetrik olan elemanlar aynı ise bu dizeye simetrik dizey denir.
$$M_{3,3} = \begin{bmatrix}1 & 2 & 3 \\ 2 & 5 & 2 \\ 7 & 2 & 1\end{bmatrix}$$

### Tam Simetrik Dizey
$a$ satır ve $a$ sütundan oluşan bir dizeyde $e_{i,j}=e_{j,i}$ ve $e_{i,j}=e_{a+1-j,a+1-i}$ ise yani asal ve yedek köşegene göre simetrik olan elemanlar aynı ise bu dizeye simetrik dizey denir.
$$M_{3,3} = \begin{bmatrix}1 & 2 & 3 \\ 2 & 5 & 2 \\ 3 & 2 & 1\end{bmatrix}$$

### Merkezcil Simetrik Dizey
$a$ satır ve $a$ sütundan oluşan bir dizeyde $e_{i,j}=e_{a+1-i,a+1-j}$ ise yani merkeze göre simetrik olan elemanlar aynı ise bu dizeye simetrik dizey denir.
$$M_{5,5} = \begin{bmatrix}1 & 2 & 3 & 4 & 5 \\ 6 & 7 & 8 & 9 & 10 \\ 11 & 12 & 13 & 12 & 11 \\ 10 & 9 & 8 & 7 & 6 \\ 5 & 4 & 3 & 2 & 1\end{bmatrix}$$
$$N_{4,4} = \begin{bmatrix}1 & 2 & 3 & 4 \\ 5 & 6 & 7 & 8 \\ 8 & 7 & 6 & 5 \\ 4 & 3 & 2 & 1\end{bmatrix}$$

### Çarpık Simetrik Dizey
$a$ satır ve $a$ sütundan oluşan bir dizeyde asal köşegen elemanları $0$ ve $e_{i,j}=-e_{j,i}$ ise yani asal köşegene göre simetrik olan elemanların oranı $-1$ ise bu dizeye çarpık simetrik dizey denir. Bir dizey çarpık simetrik dizey ise $M^T=-M$ sağlanır.
$$M_{3,3} = \begin{bmatrix}0 & 2 & -3 \\ -2 & 0 & 6 \\ 3 & -6 & 0\end{bmatrix}$$

### Satır Dizeyi
$1$ satır ve $b$ sütundan oluşan dizeylere satır dizeyi denir.
$$M_{1,4} = \begin{bmatrix}1 & 2 & 3 & 4\end{bmatrix}$$

### Sütun Dizeyi
$a$ satır ve $1$ sütundan oluşan dizeylere sütun dizeyi denir.
$$M_{3,1} = \begin{bmatrix}1 \\ 2 \\ 3\end{bmatrix}$$

## Doğrusal Denklem Sistemlerinin Dizey ile Gösterimi
Doğrusal denklem sistemleri dizey notasyonu (Katsayılar Dizeyi $\times$ Değişkenler Sütun Dizeyi $=$ Sabitler Sütun Dizeyi) kullanılarak gösterilebilir.
$$ax + by + cz = k \\ dx + ey + fz = l \\ gx + hy + jz = m \\$$
$$\begin{bmatrix}a & b & c \\ d & e & f \\ g & h & j\end{bmatrix}
\times
\begin{bmatrix}x \\ y \\ z\end{bmatrix}
=
\begin{bmatrix}k \\ l \\ m\end{bmatrix}$$

## Dizeyin Belirteci
Belirteç kare matris ile ilişkili özel bir sayıdır ve $\det(M)$ ya da $\mid M \mid$ şeklinde gösterilir.

### Belirteci Sıfır Yapan Durumlar
Bunlarla sınırlı olmamak üzere aşağıdaki durumlarda belirteç sıfırdır:
1. Bir dizeyde bir satırın (veya sütunun) tüm elemanlarının $k$ ile çarpımı yine bir satır (veya sütun) belirtiyorsa $\det(M)=0$'dır.
   1. $k=0$ olduğu durumda yani bir satırın (veya sütunun) tüm elemanları 0 ise
   2. $k=1$ olduğu durumda yeni bir satırdan (veya sütundan) birden fazla varsa

Eğer $\det(M) = 0$ ise bu dizeyin katsayılar dizeyi olarak yer aldığı doğrusal denklem sistemlerinin tek bir çözümü yoktur. ÇK $= R$ ya da ÇK $= \emptyset$ olacaktır.

### Belirtecin Özellikleri
1. Dizeyin bir satırının (veya sütununun) tüm elemanlarının $k$ ile çarpımından sonra elde edilen belirteç eski belirtecin $k$ ile çarpımına eşittir.
2. Dizeyde iki satır (veya sütun) yer değiştirirse belirteç eski belirtecin $-1$ katına eşittir.
3. Dizeyde bir satırın (veya sütunun) elemanlarının $k$ ile çarpılıp diğer bir satırın (veya sütunun) elemanlarına eklenmesi belirteci değiştirmez.
4. $\det(M) = \det(M^T)$'dir.
5. $\det(M \times N) = \det(M) * \det(N)$
6. $\frac{1}{\det(M)} = \det\left(\frac{1}{M}\right)$

### Minör ve Eş Çarpan Kavramı
$a$ satır ve $a$ sütundan oluşan $M_{a,b}$ dizeyinin $i$. satırı ve $j$. sütunu silinince geriye kalan $a-1$ satır ve $a-1$ sütunlu dizeyin belirtecine $m_{i,j}$'nin minörü denir.

$M=\begin{bmatrix}1 & 2 & 3 \\ 8 & 6 & 4 \\ 9 & 8 & 7\end{bmatrix} \qquad$
$m_{2,1} = \det(\begin{bmatrix}2 & 3 \\ 8 & 7\end{bmatrix})$

$m_{i,j}$ bir $M$ dizeyindeki $e_{i,j}$'nin minörüyse $e_{i,j}$'nin eş çarpanı $c_{i,j} = (-1)^{i+j}*m_{i,j}$ şeklinde hesaplanır.

### Belirtecin Hesaplanması
$c_{i,j}$, $e_{i,j}$'nin eş çarpanı ve $i\mid1\leqslant i \leqslant a$ olmak üzere,
$$\det(M_{a,a})=\sum^a_{j=1}c_{i,j}*e_{i,j}$$
olarak hesaplanabilir. Yine $c_{i,j}$, $e_{i,j}$'nin eş çarpanı ve $j\mid1\leqslant j \leqslant a$ olmak üzere,
$$\det(M_{a,a})=\sum^a_{i=1}c_{i,j}*e_{i,j}$$
formülü de kullanılabilir.

Görüldüğü üzere bu formül özyinelemelidir ve hesaplanması zor olabilir. Hesaplamayı kolaylaştırmak için $M=A \times B$ olacak şekilde iki dizeye ayrıştırmak, "Belirtecin Özellikleri" başlığında açıklanan belirteci değiştirmeyen değişiklikleri uygulamak veya en fazla $0$ içeren satır veya sütunu seçmek hesaplamayı kolaylaştırabilir.

<br>
Ayrıca bazı durumlar için kısayollar türetilmiştir, hepsinin yukarıdaki formülü temel aldığı görülebilir.

$a=1$ ise $\det(M_{1,1})=e_{1,1}$'dir.

$a=2$ ise $\det(M_{2,2})=e_{1,1}*e_{2,2} - e_{1,2}*e_{2,1}$'dir.

$a=3$ ise $\det(\begin{bmatrix}a&b&c\\d&e&f\\g&h&i\end{bmatrix})=a(ei-fh)-b(di-fg)+c(dh-eg)=aei+bfg+cdh-(cef+bdi+afh)$'dir.

## Ters Dizey
$M_{a,a}$'in tersi $M^{-1}_{a,a}$ şeklinde gösterilir. Ters dizey, dizeylerde bölme yapmak için kullanılabilir.

### Ters Dizeyin Özellikleri
$M$ tersi alınabilir bir dizey ise
1. $M_{a,a}^{-1} \times M_{a,a} = M_{a,a} \times M^{-1}_{a,a}=I_{a,a}$
2. $N$ tersi alınabilen bir dizey ise $(M\times N)^{-1} = M^{-1} \times N^{-1}$'dir.
3. $(M^{-1})^{-1} = M$'dir.
4. $(M^T)^{-1} = (M^{-1})^T$'dir.
5. $k\neq 0$ ise $k\cdot M = (1/k)\cdot M^{-1}$'dir.

### Ek Dizeyin Belirteç ve Ters Dizey ile İlişkisi
Her dizey tanımlandığında elemanları dizeyin elemanlarının eş çarpanı olan ve $\mathop{adj}(M)$ şeklinde gösterilen bir ek dizey tanımlar. $C$ eş çarpan dizeyi olmak üzere $\mathop{adj}(M)=C^T$'dir ve $m_{i,j}$, $M$'nin minörü olmak üzere $C^T_{i,j} = (-1)^{i+j}*m_{j,i}$'dir.

$M = \begin{bmatrix}1 & 3 \\ 5 & 7\end{bmatrix} \qquad$
$C = \begin{bmatrix}7 & -5 \\ -3 & 1\end{bmatrix} \qquad$
$\mathop{adj}(M)= C^T = \begin{bmatrix}7 & -3 \\ -5 & 1\end{bmatrix} \qquad$

<br>
Ek dizeyler ile dizeyin belirteci arasındaki ilişki aşağıdaki gibidir. Bu eşitlik eş çarpan dizeyinin tanımı, belirtecin tanımı ve dizey çarpmasının tanımı gereği sağlanmaktadır.

$$\mathop{adj}(M)\times M = M\times \mathop{adj}(M) = \det(M)\cdot I$$

Ayrıca bu eşitliğe çeşitli işlemler uygulanarak $M^{-1}$ elde edilebilir.

$$\frac{M\times \mathop{adj}(M)}{det(M)} = I$$
$$\frac{\mathop{adj}(M)}{det(M)} = \frac{I}{M^{-1}} = M^{-1}$$

Buradan $M$ dizeyinin terslenebilir olması için $\det(M)\neq 0$ olması gerektiği görülür.

## Ters Dizey Kullanarak Doğrusal Denklem Sistemlerinin Çözümü
Doğrusal denklem sistemlerinin dizey notasyonunun $KX=S$ şeklinde olduğu düşünülürse $K^{-1}\times K\times X=K^{-1} \times S$ yani $X = K^{-1}\times S$ olacaktır.
$$3x + 4y = 11 \\ 4x + 5y = 14$$
$$\begin{bmatrix}x \\ y\end{bmatrix} =
\left(\frac{\mathop{adj}\left(\begin{bmatrix}3 & 4 \\ 4 & 5\end{bmatrix}\right)}{\det\left(\begin{bmatrix}3 & 4 \\ 4 & 5\end{bmatrix}\right)}\right) \times \begin{bmatrix}11 \\ 14\end{bmatrix} 
= \begin{bmatrix}1 \\ 2\end{bmatrix}$$

## Dizeylerin Kullanım Alanları
Matematikte doğrusal denklem sistemlerinin çözümü, paralel kenarın alanının hesaplanması (paralelkenarı oluşturan vektörleri satır kabul eden dizeyin belirteci bu paralelkenarın alanına eşittir) gibi birçok alanda dizey ve belirteç kavramı işe yarar fakat aslında dizeyler birer tablodur ve elemanları gerçel sayılar, karmaşık sayılar ve sayı olmayan nesneler olabilir, örneğin bir apartmanda oturan kişilerin hangi kat ve hangi dairede oturduğu bilgisini içeren tablo veya havaalanlarındaki seyahat saatleri tablosu bir dizey tanımlayabilir.

Günlük hayatta ve matematikte kullanımlarının yanı sıra bilgisayar bilimlerinde de sıkça kullanılan dizey ve belirteç kavramlarının uygulamalarına örnek olarak hata düzeltme algoritmaları, veri şifreleme algoritmaları, ekonomik denge analizleri, veri anonimleştirme ve genelleştirme işlemleri, sosyal ağ ve sanal reklamcılıkta öneri sistemleri, derin-öğrenme yöntemlerinin çok boyutlu ve ilişkili dizi ve dizeyler ile geliştirilmesi gibi şeyler verilebilir.