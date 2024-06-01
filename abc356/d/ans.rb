def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

MOD = 998244353
N, M = getis
# 1桁目 → 周期2、前半1がオン
# n桁目 → 周期2**n, 前半 2**(n-1)がオフ
ans = 0
rank = N.to_s(2).size
1.upto(rank) do |r|
  # p [r, ans]
  next if (1<<(r-1)) & M == 0

  t = 2**r
  tn = (N+1)/t
  ans += tn * (t/2)
  # p ["r", r, "t", t, "tn", tn, ans]
  ans %= MOD
  rest = (N+1) % t
  tmp = rest - (t/2)
  # p [rest, tmp]
  if tmp > 0
    ans += tmp
    ans %= MOD
  end
end

puts ans
