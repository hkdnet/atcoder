def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

N, M, K = getis

tests = M.times.map do
  _, *arr = getl.split(" ")
  keys = arr[...-1]
  result = arr[-1]
  tmp = 0
  keys.each do |e|
    i = e.to_i - 1
    tmp = tmp | (1<<i)
  end
  { keys: tmp, result: result == 'o' }
end

ans = 0

(1<<N).times do |pat|
  f = tests.all? do |h|
    keys = h[:keys]
    result = h[:result]
    num = pat & keys
    opened = num.to_s(2).count("1") >= K
    opened == result
  end
  if f
    ans += 1
    # p (("0"*N)+pat.to_s(2))[-N..]
  end
end

puts ans
