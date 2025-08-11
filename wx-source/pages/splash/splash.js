Page({
    data: {
        isExiting: !1,
        animationState: {
            isEntering: !0,
            contentReady: !1
        }
    },
    onLoad: function() {
        this.clearEasterCount(), this.initPageAnimation(), this.startAnimationSequence();
    },
    clearEasterCount: function() {
        try {
            wx.setStorageSync("easterCount", 0);
        } catch (t) {}
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
            }, 200);
        });
    },
    startAnimationSequence: function() {
        var t = this;
        setTimeout(function() {
            t.startExitAnimation();
        }, 1500);
    },
    startExitAnimation: function() {
        this.setData({
            isExiting: !0
        }), setTimeout(function() {
            wx.redirectTo({
                url: "/pages/index/index"
            });
        }, 300);
    },
    onTap: function() {
        wx.redirectTo({
            url: "/pages/index/index"
        });
    },
    onShow: function() {},
    onHide: function() {}
});