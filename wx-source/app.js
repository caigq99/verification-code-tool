App({
    globalData: {},
    onLaunch: function() {
        this.clearEasterCount();
        var t = wx.getStorageSync("logs") || [];
        t.unshift(Date.now()), wx.setStorageSync("logs", t), wx.login({
            success: function(t) {}
        });
    },
    clearEasterCount: function() {
        try {
            wx.setStorageSync("easterCount", 0);
        } catch (t) {}
    }
});