class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:
        starts = sorted(s for s, _ in intervals)
        ends = sorted(e for _, e in intervals)
        rooms = j = 0
        for s in starts:
            if ends[j] <= s:
                j += 1
            else:
                rooms += 1
        return rooms
