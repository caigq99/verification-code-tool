Page({
    data: {
        animationState: {
            isEntering: !0,
            isExiting: !1,
            contentReady: !1
        },
        systemInfo: {
            statusBarHeight: 44,
            menuButtonHeight: 32,
            menuButtonTop: 0,
            backButtonTop: 0,
            backButtonLeft: 16
        }
    },
    onLoad: function() {
        this.clearEasterCount(), this.getSystemInfo(), this.initPageAnimation();
    },
    clearEasterCount: function() {
        try {
            wx.setStorageSync("easterCount", 0);
        } catch (t) {}
    },
    getSystemInfo: function() {
        var t = wx.getSystemInfoSync(), n = wx.getMenuButtonBoundingClientRect(), i = n.top + 6, a = t.windowWidth - n.right + 8;
        this.setData({
            "systemInfo.statusBarHeight": t.statusBarHeight,
            "systemInfo.menuButtonHeight": n.height,
            "systemInfo.menuButtonTop": n.top,
            "systemInfo.backButtonTop": i,
            "systemInfo.backButtonLeft": a
        });
    },
    initPageAnimation: function() {
        var t = this;
        wx.nextTick(function() {
            t.setData({
                "animationState.isEntering": !0
            }), setTimeout(function() {
                t.setData({
                    "animationState.contentReady": !0,
                    "animationState.isEntering": !1
                });
            }, 80);
        });
    },
    selectSoftware: function(t) {
        var n = t.currentTarget.dataset.software;
        this.navigateToValidityPage(n);
    },
    navigateToValidityPage: function(t) {
        this.startExitAnimation().then(function() {
            wx.navigateTo({
                url: "/pages/validity/validity?software=".concat(t),
                fail: function() {
                    wx.showToast({
                        title: "跳转失败",
                        icon: "error",
                        duration: 2e3
                    });
                }
            });
        });
    },
    startExitAnimation: function() {
        var t = this;
        return new Promise(function(n) {
            t.setData({
                "animationState.isExiting": !0
            }), setTimeout(function() {
                n(!0);
            }, 400);
        });
    },
    navigateBack: function() {
        this.startExitAnimation().then(function() {
            wx.navigateBack();
        });
    },
    onShow: function() {
        this.setData({
            "animationState.isExiting": !1
        });
    },
    onHide: function() {},
    onUnload: function() {
        this.setData({
            "animationState.isEntering": !1,
            "animationState.isExiting": !1,
            "animationState.contentReady": !1
        });
    }
});