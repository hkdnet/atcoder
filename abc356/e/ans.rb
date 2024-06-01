def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }


def bin_search(l, r)
  loop do
    idx = (l + r)/2
    return idx if idx == l || idx == r
    f = yield idx
    if f
      l = idx
    else
      r = idx
    end
  end
end


N = geti
A = getis.sort
ans = 0
threshold = 5
# p A
0.upto(N-1) do |l|
  r = N - 1
  while r > l
    tmp = A[r]/A[l]
    ans += tmp

    if (r-threshold)>l && A[r] - A[r-threshold] < l
      nx = bin_search(l+1, r) do |i|
        A[i]/A[l] != tmp
      end
      nx += 1

      skip = r-nx
      if skip > 1
        ans += (skip-1)*tmp
        # p ["skipped", r, nx, skip*tmp]
        r = nx
      else
        # puts "skip aborted"
        r -=1
      end
    else
      r -= 1
      next
    end
  end
  # p [l, ans]
end

puts ans
