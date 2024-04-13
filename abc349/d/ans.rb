def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

L, R = getis

tmp = L.to_s(2)
arr = []
arr << tmp

r2 = R.to_s(2)

if tmp == "0"
  tmp = "1" + ("0" * (r2.size - 1))
  arr << tmp
end

while tmp != r2 do
  # p tmp # debug

  if tmp.size == r2.size
    take_p = true
    s = tmp.chars.zip(r2.chars).map do |tc, tr|
      if take_p
        if tc != tr
          take_p = false
        end
        tr
      else
        "0"
      end
    end
    nx = s.join("")
    if nx <= tmp
      p [t, s, nx]
      raise "nx = #{nx}"
    end
    arr << nx
    tmp = nx
  else
    if tmp.match?(/\A1+0*\z/)
      nx = "1" + ("0" * tmp.size)
      arr << nx
      tmp = nx
    else
      tn = tmp.chars.take_while {|c| c == "1" }.join("")
      nx = "#{tn}1#{"0"*(tmp.size - tn.size-1)}"
      arr << nx
      tmp = nx
    end
  end
end

ans = arr.map {|e| e.to_i(2)}.each_cons(2).to_a
puts ans.size
ans.each do |l, r|
  puts "#{l} #{r}"
end
