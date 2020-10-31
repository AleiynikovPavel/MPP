package com.company;

class RustJni {

    private static native int sumPerfectSquare(int [] arr);

    private static native int sumPerfectSquarePar(int [] arr);

    private static native int sumPerfectSquarePar(int [] arr, int thread_count);

    static {
        System.loadLibrary("libmpp");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        System.out.println(RustJni.sumPerfectSquare(new int[] {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}));
        System.out.println(RustJni.sumPerfectSquarePar(new int[] {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}));
        System.out.println(RustJni.sumPerfectSquarePar(new int[] {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}, 2));
    }
}
