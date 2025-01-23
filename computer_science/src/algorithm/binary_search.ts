/**
 * Search through a sorted array for a target value.
 * @returns index of target value, or null if not found
 * */
const binarySearch = (sortedArr: number[], target: number): number | null => {
    if (sortedArr.length === 0) {
        return null;
    }

    let left = 0;
    let right = sortedArr.length - 1;

    while (true) {
        // last 1 or 2 elements
        if (right - left <= 1) {
            if (sortedArr[left] === target) return left;
            if (sortedArr[right] === target) return right;
            return null;
        }

        // 3 or more elements
        const testedPosition = Math.trunc((left + right) / 2);
        const testedValue = sortedArr[testedPosition];

        if (target === testedValue) {
            return testedPosition;
        } else if (target < testedValue) {
            right = testedPosition - 1;
        } else {
            left = testedPosition + 1;
        }
    }
};
