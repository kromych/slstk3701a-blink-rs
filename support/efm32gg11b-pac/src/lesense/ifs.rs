#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `CH0` writer - Set CH0 Interrupt Flag"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Set CH1 Interrupt Flag"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Set CH2 Interrupt Flag"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Set CH3 Interrupt Flag"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Set CH4 Interrupt Flag"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Set CH5 Interrupt Flag"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Set CH6 Interrupt Flag"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Set CH7 Interrupt Flag"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Set CH8 Interrupt Flag"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Set CH9 Interrupt Flag"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Set CH10 Interrupt Flag"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Set CH11 Interrupt Flag"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - Set CH12 Interrupt Flag"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - Set CH13 Interrupt Flag"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - Set CH14 Interrupt Flag"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - Set CH15 Interrupt Flag"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCOMPLETE` writer - Set SCANCOMPLETE Interrupt Flag"]
pub type ScancompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEC` writer - Set DEC Interrupt Flag"]
pub type DecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECERR` writer - Set DECERR Interrupt Flag"]
pub type DecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFDATAV` writer - Set BUFDATAV Interrupt Flag"]
pub type BufdatavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFLEVEL` writer - Set BUFLEVEL Interrupt Flag"]
pub type BuflevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOF` writer - Set BUFOF Interrupt Flag"]
pub type BufofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOF` writer - Set CNTOF Interrupt Flag"]
pub type CntofW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set CH0 Interrupt Flag"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, IfsSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set CH1 Interrupt Flag"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, IfsSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set CH2 Interrupt Flag"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, IfsSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Set CH3 Interrupt Flag"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, IfsSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Set CH4 Interrupt Flag"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, IfsSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Set CH5 Interrupt Flag"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, IfsSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Set CH6 Interrupt Flag"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, IfsSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Set CH7 Interrupt Flag"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, IfsSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Set CH8 Interrupt Flag"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<'_, IfsSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Set CH9 Interrupt Flag"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<'_, IfsSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Set CH10 Interrupt Flag"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<'_, IfsSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Set CH11 Interrupt Flag"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<'_, IfsSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Set CH12 Interrupt Flag"]
    #[inline(always)]
    pub fn ch12(&mut self) -> Ch12W<'_, IfsSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Set CH13 Interrupt Flag"]
    #[inline(always)]
    pub fn ch13(&mut self) -> Ch13W<'_, IfsSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Set CH14 Interrupt Flag"]
    #[inline(always)]
    pub fn ch14(&mut self) -> Ch14W<'_, IfsSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Set CH15 Interrupt Flag"]
    #[inline(always)]
    pub fn ch15(&mut self) -> Ch15W<'_, IfsSpec> {
        Ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - Set SCANCOMPLETE Interrupt Flag"]
    #[inline(always)]
    pub fn scancomplete(&mut self) -> ScancompleteW<'_, IfsSpec> {
        ScancompleteW::new(self, 16)
    }
    #[doc = "Bit 17 - Set DEC Interrupt Flag"]
    #[inline(always)]
    pub fn dec(&mut self) -> DecW<'_, IfsSpec> {
        DecW::new(self, 17)
    }
    #[doc = "Bit 18 - Set DECERR Interrupt Flag"]
    #[inline(always)]
    pub fn decerr(&mut self) -> DecerrW<'_, IfsSpec> {
        DecerrW::new(self, 18)
    }
    #[doc = "Bit 19 - Set BUFDATAV Interrupt Flag"]
    #[inline(always)]
    pub fn bufdatav(&mut self) -> BufdatavW<'_, IfsSpec> {
        BufdatavW::new(self, 19)
    }
    #[doc = "Bit 20 - Set BUFLEVEL Interrupt Flag"]
    #[inline(always)]
    pub fn buflevel(&mut self) -> BuflevelW<'_, IfsSpec> {
        BuflevelW::new(self, 20)
    }
    #[doc = "Bit 21 - Set BUFOF Interrupt Flag"]
    #[inline(always)]
    pub fn bufof(&mut self) -> BufofW<'_, IfsSpec> {
        BufofW::new(self, 21)
    }
    #[doc = "Bit 22 - Set CNTOF Interrupt Flag"]
    #[inline(always)]
    pub fn cntof(&mut self) -> CntofW<'_, IfsSpec> {
        CntofW::new(self, 22)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
