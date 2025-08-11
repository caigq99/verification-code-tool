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
        },
        showRealEaster: !1,
        videoCompleted: !1,
        easterMessage: "恭喜！观看完整视频获得彩蛋奖励",
        basicReward: {
            domain: "yanabc.com"
        },
        easterContent: {
            domain: "yanabc.com",
            email: "yaugment@mailto.plus",
            pin: "yaugment"
        },
        rewardAnimation: {
            showRewardBanner: !1,
            showContent: !1
        },
        easterCount: 0,
        remainingCount: 4
    },
    onLoad: function(t) {
        this.getSystemInfo(), this.checkVideoCompletedStatus(t), this.initPageAnimation();
    },
    checkVideoCompletedStatus: function(t) {
        t && "true" === t.videoCompleted ? (this.setData({
            videoCompleted: !0
        }), this.showRewardAnimation(), this.checkEasterCount()) : this.checkEasterCount();
    },
    checkEasterCount: function() {
        try {
            var t = wx.getStorageSync("easterCount") || 0;
            t++, wx.setStorageSync("easterCount", t);
            var n = Math.max(0, 4 - t);
            this.setData({
                easterCount: t,
                remainingCount: n
            }), t >= 4 && (this.setData({
                showRealEaster: !0
            }), wx.setStorageSync("easterCount", 0));
        } catch (t) {}
    },
    showRewardAnimation: function() {
        var t = this;
        setTimeout(function() {
            t.setData({
                "rewardAnimation.showRewardBanner": !0
            });
        }, 500), setTimeout(function() {
            t.setData({
                "rewardAnimation.showContent": !0
            });
        }, 1500);
    },
    getSystemInfo: function() {
        var t = wx.getSystemInfoSync(), n = wx.getMenuButtonBoundingClientRect(), e = n.top + 6, a = t.windowWidth - n.right + 8;
        this.setData({
            "systemInfo.statusBarHeight": t.statusBarHeight,
            "systemInfo.menuButtonHeight": n.height,
            "systemInfo.menuButtonTop": n.top,
            "systemInfo.backButtonTop": e,
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
            }, 150);
        });
    },
    startExitAnimation: function() {
        var t = this;
        return new Promise(function(n) {
            t.setData({
                "animationState.isExiting": !0
            }), setTimeout(function() {
                n(!0);
            }, 200);
        });
    },
    navigateBack: function() {
        this.startExitAnimation().then(function() {
            wx.navigateBack();
        });
    },
    copyText: function(t) {
        var n = t.currentTarget.dataset.text, e = t.currentTarget.dataset.label;
        wx.setClipboardData({
            data: n,
            success: function() {
                wx.showToast({
                    title: "".concat(e, "已复制"),
                    icon: "success",
                    duration: 1500
                });
            },
            fail: function() {
                wx.showToast({
                    title: "复制失败",
                    icon: "error",
                    duration: 1500
                });
            }
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