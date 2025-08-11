require("../../@babel/runtime/helpers/Arrayincludes");

getApp();

Page({
    data: {
        animationState: {
            isEntering: !0,
            isExiting: !1,
            contentReady: !1
        },
        notices: [],
        currentNoticeIndex: 0,
        noticeLoading: !0,
        noticeError: !1
    },
    rewardedVideoAd: null,
    currentAdTarget: "",
    adsInitialized: !1,
    isAdPlaying: !1,
    noticeTimer: null,
    onLoad: function() {
        this.initPageAnimation(), this.adsInitialized || this.initRewardedVideoAd(), this.fetchNotices();
    },
    fetchNotices: function() {
        var t = this;
        this.setData({
            noticeLoading: !0,
            noticeError: !1
        }), wx.request({
            url: "https://app.yan.vin/XiaoChengXu/notice.json",
            method: "GET",
            timeout: 1e4,
            success: function(e) {
                if (200 === e.statusCode && e.data) try {
                    var i = [];
                    Array.isArray(e.data) ? i = e.data.filter(function(t) {
                        return t && t.content;
                    }) : e.data.notices && Array.isArray(e.data.notices) ? i = e.data.notices.filter(function(t) {
                        return t && t.content;
                    }) : e.data.content && (i = [ e.data ]), i.length > 0 ? (t.setData({
                        notices: i,
                        noticeLoading: !1,
                        noticeError: !1,
                        currentNoticeIndex: 0
                    }), i.length > 1 && t.startNoticeCarousel()) : t.handleNoticeError("公告内容为空");
                } catch (e) {
                    t.handleNoticeError("解析公告数据失败");
                } else t.handleNoticeError("请求失败 (".concat(e.statusCode, ")"));
            },
            fail: function() {
                t.handleNoticeError("网络请求失败");
            }
        });
    },
    handleNoticeError: function(t) {
        this.setData({
            notices: [ {
                content: "啊哦，获取公告失败了呢，或许是网络问题"
            } ],
            noticeLoading: !1,
            noticeError: !0,
            currentNoticeIndex: 0
        });
    },
    startNoticeCarousel: function() {
        var t = this;
        this.noticeTimer && clearInterval(this.noticeTimer), this.noticeTimer = setInterval(function() {
            var e = t.data.notices;
            if (e.length > 1) {
                var i = (t.data.currentNoticeIndex + 1) % e.length;
                t.setData({
                    currentNoticeIndex: i
                });
            }
        }, 5e3);
    },
    stopNoticeCarousel: function() {
        this.noticeTimer && (clearInterval(this.noticeTimer), this.noticeTimer = null);
    },
    initRewardedVideoAd: function() {
        var t = this;
        wx.createRewardedVideoAd && (this.rewardedVideoAd = wx.createRewardedVideoAd({
            adUnitId: "adunit-ac487887561b63a2"
        }), this.rewardedVideoAd.onLoad(function() {
            console.log("激励广告加载成功");
        }), this.rewardedVideoAd.onError(function(t) {
            console.error("激励广告加载失败:", t);
        }), this.rewardedVideoAd.onClose(function(e) {
            t.handleAdClose(e);
        }), this.rewardedVideoAd.load().catch(function(t) {
            console.error("初始预加载失败:", t);
        }), this.adsInitialized = !0);
    },
    handleAdClose: function(t) {
        var e = this;
        if (this.isAdPlaying = !1, wx.hideToast(), t && t.isEnded) {
            var i = this.currentAdTarget.includes("code") ? "验证码" : "彩蛋内容";
            wx.showToast({
                title: "恭喜获得".concat(i, "！"),
                icon: "success",
                duration: 1500
            }), setTimeout(function() {
                e.startExitAnimation().then(function() {
                    var t = e.currentAdTarget;
                    t.includes("easter") && (t += "?videoCompleted=true"), wx.navigateTo({
                        url: t
                    }), e.currentAdTarget = "";
                });
            }, 1500);
        } else this.currentAdTarget = "", wx.showModal({
            title: "视频未观看完整",
            content: "需要观看完整视频广告才能获得奖励，请重新尝试",
            showCancel: !1,
            confirmText: "知道了"
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
            }, 100);
        });
    },
    startExitAnimation: function() {
        var t = this;
        return new Promise(function(e) {
            t.setData({
                "animationState.isExiting": !0
            }), setTimeout(function() {
                e(!0);
            }, 200);
        });
    },
    showRewardedVideoAd: function(t) {
        var e = this;
        this.currentAdTarget = t;
        var i = t.includes("code") ? "验证码" : "彩蛋内容";
        wx.showModal({
            title: "观看视频获取奖励",
            content: "观看完整视频广告后即可获得".concat(i, "，是否继续？"),
            confirmText: "观看视频",
            cancelText: "取消",
            success: function(t) {
                t.confirm ? e.playRewardedVideoAd() : e.currentAdTarget = "";
            }
        });
    },
    playRewardedVideoAd: function() {
        var t = this;
        this.isAdPlaying ? wx.showToast({
            title: "广告正在加载中，请稍候...",
            icon: "none",
            duration: 1500
        }) : (this.isAdPlaying = !0, wx.showToast({
            title: "正在加载广告...",
            icon: "loading",
            duration: 15e3
        }), this.rewardedVideoAd ? this.rewardedVideoAd.show().then(function() {
            console.log("广告show()调用成功"), wx.hideToast();
        }).catch(function() {
            var e;
            wx.showToast({
                title: "正在重新加载广告...",
                icon: "loading",
                duration: 5e3
            }), null === (e = t.rewardedVideoAd) || void 0 === e || e.load().then(function() {
                var e;
                return null === (e = t.rewardedVideoAd) || void 0 === e ? void 0 : e.show();
            }).then(function() {
                console.log("广告重新加载并show()调用成功"), wx.hideToast();
            }).catch(function(e) {
                console.error("广告加载失败:", e), wx.hideToast(), wx.showToast({
                    title: "广告加载失败，请稍后重试或查看文档说明解决",
                    icon: "none",
                    duration: 2e3
                }), t.currentAdTarget = "", t.isAdPlaying = !1;
            });
        }) : (wx.hideToast(), this.startExitAnimation().then(function() {
            wx.navigateTo({
                url: t.currentAdTarget
            }), t.currentAdTarget = "", t.isAdPlaying = !1;
        })));
    },
    getVerificationCode: function() {
        this.startExitAnimation().then(function() {
            wx.navigateTo({
                url: "/pages/software-select/software-select"
            });
        });
    },
    showAppInfo: function() {
        this.startExitAnimation().then(function() {
            wx.navigateTo({
                url: "/pages/app-info/app-info"
            });
        });
    },
    showEasterEgg: function() {
        this.showRewardedVideoAd("/pages/easter/easter");
    },
    navigateToValidityPage: function() {
        this.startExitAnimation().then(function() {
            wx.navigateTo({
                url: "/pages/validity-select/validity-select"
            });
        });
    },
    onShow: function() {
        this.setData({
            "animationState.isExiting": !1
        }), this.rewardedVideoAd && this.rewardedVideoAd.load().catch(function(t) {
            console.log("预加载失败，但不影响使用:", t);
        });
    },
    onHide: function() {},
    onUnload: function() {
        this.setData({
            "animationState.isEntering": !1,
            "animationState.isExiting": !1,
            "animationState.contentReady": !1
        }), this.rewardedVideoAd && (this.rewardedVideoAd.destroy && this.rewardedVideoAd.destroy(), 
        this.rewardedVideoAd = null), this.stopNoticeCarousel(), this.currentAdTarget = "", 
        this.adsInitialized = !1;
    }
});