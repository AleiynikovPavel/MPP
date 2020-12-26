package com.company;

class RustJni {

    private static native int sumPerfectSquare(int [] arr);

    private static native int sumPerfectSquarePar(int [] arr);

    private static native int sumPerfectSquarePar(int [] arr, int thread_count);

    static {
        System.loadLibrary("libmpp");
    }

    public static void main(String[] args) {
        int N = 1_000_000;
        int [] arr = new int [N];
        for (int i = 0; i < N; i++) {
            arr[i] = 1;
        }
        System.out.println(RustJni.sumPerfectSquare(arr));
        System.out.println(RustJni.sumPerfectSquarePar(arr));
        System.out.println(RustJni.sumPerfectSquarePar(arr, 2));
    }
}
