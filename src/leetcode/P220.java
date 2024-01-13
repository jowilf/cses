package leetcode;

import java.util.TreeSet;

public class P220 {

    public boolean containsNearbyAlmostDuplicate(int[] nums, int indexDiff, int valueDiff) {
        TreeSet<Integer> tree = new TreeSet<>();
        for (int i = 0; i < nums.length; i++) {
            if (i > indexDiff) {
                tree.remove(nums[i - indexDiff - 1]);
                // System.out.println("remove " + i + "->" + nums[i - indexDiff - 1]);
            }
            Integer lower = tree.lower(nums[i] + 1);
            Integer higher = tree.higher(nums[i] - 1);
            if ((lower != null && (nums[i] - lower) <= valueDiff)
                    || (higher != null && (higher - nums[i]) <= valueDiff))
                return true;
            tree.add(nums[i]);
            // System.out.println(tree);
        }
        return false;
    }

    public static void main(String[] args) {
        System.out.println(new P220().containsNearbyAlmostDuplicate(new int[] { 8, 7, 15, 1, 6, 1, 9, 15 }, 1, 3));
    }

}