def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

N, M, K = getis

tests = M.times.map do
  _, *arr = getl.split(" ")
  keys = arr[...-1]
  result = arr[-1]
  keys.map! { |e| e.to_i - 1 }
  { keys:, result: result == 'o' }
end

ans = 0

(2**N).times do |pat|
  next if pat.to_s(2).count("1") < K
  f = tests.all? do |h|
    keys = h[:keys]
    result = h[:result]
    tmp = 0
    keys.each do |i|
      tmp = tmp | (1<<i)
    end
    num = pat & tmp
    opened = num.to_s(2).count("1") >= K
    opened == result
  end
  if f
    ans += 1
    # p ("000"+pat.to_s(2))[-3..]
  end

end

puts ans

