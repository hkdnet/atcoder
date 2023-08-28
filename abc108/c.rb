N, K = gets.chomp.split(" ").map(&:to_i)

# a を決めたら a + b = 0 mod K より b mod K は一意に決まる。
# a が 0 のとき b, c も 0
# それ以外のときは b = K - a となる。 K が偶数のときに a mod K と b mod K がかぶるケースがある
# かぶったときは C も同じところから選ぶ
# K が奇数のときには全部 0 mod K からしか選べない

cnt = ->(a) {
  ret = N / K
  if a != 0 && a <= N % K
    ret + 1
  else
    ret
  end
}

if K % 2 == 0
  ans = 0
  K.times do |ma|
    mb = (K - ma) % K

    ca = cnt[ma]

    if ma == mb
      # c も同じなので個数を数えて3乗
      ans += ca**3
    end
  end
  puts ans
else
  puts cnt[0]**3
end
