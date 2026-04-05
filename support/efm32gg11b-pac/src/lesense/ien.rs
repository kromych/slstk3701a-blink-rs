#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `CH0` reader - CH0 Interrupt Enable"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - CH0 Interrupt Enable"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - CH1 Interrupt Enable"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - CH1 Interrupt Enable"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - CH2 Interrupt Enable"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - CH2 Interrupt Enable"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - CH3 Interrupt Enable"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - CH3 Interrupt Enable"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` reader - CH4 Interrupt Enable"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH4` writer - CH4 Interrupt Enable"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` reader - CH5 Interrupt Enable"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH5` writer - CH5 Interrupt Enable"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` reader - CH6 Interrupt Enable"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH6` writer - CH6 Interrupt Enable"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` reader - CH7 Interrupt Enable"]
pub type Ch7R = crate::BitReader;
#[doc = "Field `CH7` writer - CH7 Interrupt Enable"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` reader - CH8 Interrupt Enable"]
pub type Ch8R = crate::BitReader;
#[doc = "Field `CH8` writer - CH8 Interrupt Enable"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` reader - CH9 Interrupt Enable"]
pub type Ch9R = crate::BitReader;
#[doc = "Field `CH9` writer - CH9 Interrupt Enable"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` reader - CH10 Interrupt Enable"]
pub type Ch10R = crate::BitReader;
#[doc = "Field `CH10` writer - CH10 Interrupt Enable"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` reader - CH11 Interrupt Enable"]
pub type Ch11R = crate::BitReader;
#[doc = "Field `CH11` writer - CH11 Interrupt Enable"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` reader - CH12 Interrupt Enable"]
pub type Ch12R = crate::BitReader;
#[doc = "Field `CH12` writer - CH12 Interrupt Enable"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` reader - CH13 Interrupt Enable"]
pub type Ch13R = crate::BitReader;
#[doc = "Field `CH13` writer - CH13 Interrupt Enable"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` reader - CH14 Interrupt Enable"]
pub type Ch14R = crate::BitReader;
#[doc = "Field `CH14` writer - CH14 Interrupt Enable"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` reader - CH15 Interrupt Enable"]
pub type Ch15R = crate::BitReader;
#[doc = "Field `CH15` writer - CH15 Interrupt Enable"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCOMPLETE` reader - SCANCOMPLETE Interrupt Enable"]
pub type ScancompleteR = crate::BitReader;
#[doc = "Field `SCANCOMPLETE` writer - SCANCOMPLETE Interrupt Enable"]
pub type ScancompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEC` reader - DEC Interrupt Enable"]
pub type DecR = crate::BitReader;
#[doc = "Field `DEC` writer - DEC Interrupt Enable"]
pub type DecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECERR` reader - DECERR Interrupt Enable"]
pub type DecerrR = crate::BitReader;
#[doc = "Field `DECERR` writer - DECERR Interrupt Enable"]
pub type DecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFDATAV` reader - BUFDATAV Interrupt Enable"]
pub type BufdatavR = crate::BitReader;
#[doc = "Field `BUFDATAV` writer - BUFDATAV Interrupt Enable"]
pub type BufdatavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFLEVEL` reader - BUFLEVEL Interrupt Enable"]
pub type BuflevelR = crate::BitReader;
#[doc = "Field `BUFLEVEL` writer - BUFLEVEL Interrupt Enable"]
pub type BuflevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOF` reader - BUFOF Interrupt Enable"]
pub type BufofR = crate::BitReader;
#[doc = "Field `BUFOF` writer - BUFOF Interrupt Enable"]
pub type BufofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOF` reader - CNTOF Interrupt Enable"]
pub type CntofR = crate::BitReader;
#[doc = "Field `CNTOF` writer - CNTOF Interrupt Enable"]
pub type CntofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0 Interrupt Enable"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Interrupt Enable"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Interrupt Enable"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Interrupt Enable"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4 Interrupt Enable"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5 Interrupt Enable"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6 Interrupt Enable"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7 Interrupt Enable"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8 Interrupt Enable"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9 Interrupt Enable"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10 Interrupt Enable"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11 Interrupt Enable"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CH12 Interrupt Enable"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CH13 Interrupt Enable"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CH14 Interrupt Enable"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CH15 Interrupt Enable"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCANCOMPLETE Interrupt Enable"]
    #[inline(always)]
    pub fn scancomplete(&self) -> ScancompleteR {
        ScancompleteR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DEC Interrupt Enable"]
    #[inline(always)]
    pub fn dec(&self) -> DecR {
        DecR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DECERR Interrupt Enable"]
    #[inline(always)]
    pub fn decerr(&self) -> DecerrR {
        DecerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BUFDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BufdatavR {
        BufdatavR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BUFLEVEL Interrupt Enable"]
    #[inline(always)]
    pub fn buflevel(&self) -> BuflevelR {
        BuflevelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BUFOF Interrupt Enable"]
    #[inline(always)]
    pub fn bufof(&self) -> BufofR {
        BufofR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CNTOF Interrupt Enable"]
    #[inline(always)]
    pub fn cntof(&self) -> CntofR {
        CntofR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Interrupt Enable"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, IenSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - CH1 Interrupt Enable"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, IenSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - CH2 Interrupt Enable"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, IenSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - CH3 Interrupt Enable"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, IenSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - CH4 Interrupt Enable"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, IenSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - CH5 Interrupt Enable"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, IenSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - CH6 Interrupt Enable"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, IenSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - CH7 Interrupt Enable"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, IenSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - CH8 Interrupt Enable"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<'_, IenSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - CH9 Interrupt Enable"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<'_, IenSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - CH10 Interrupt Enable"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<'_, IenSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - CH11 Interrupt Enable"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<'_, IenSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - CH12 Interrupt Enable"]
    #[inline(always)]
    pub fn ch12(&mut self) -> Ch12W<'_, IenSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - CH13 Interrupt Enable"]
    #[inline(always)]
    pub fn ch13(&mut self) -> Ch13W<'_, IenSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - CH14 Interrupt Enable"]
    #[inline(always)]
    pub fn ch14(&mut self) -> Ch14W<'_, IenSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - CH15 Interrupt Enable"]
    #[inline(always)]
    pub fn ch15(&mut self) -> Ch15W<'_, IenSpec> {
        Ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - SCANCOMPLETE Interrupt Enable"]
    #[inline(always)]
    pub fn scancomplete(&mut self) -> ScancompleteW<'_, IenSpec> {
        ScancompleteW::new(self, 16)
    }
    #[doc = "Bit 17 - DEC Interrupt Enable"]
    #[inline(always)]
    pub fn dec(&mut self) -> DecW<'_, IenSpec> {
        DecW::new(self, 17)
    }
    #[doc = "Bit 18 - DECERR Interrupt Enable"]
    #[inline(always)]
    pub fn decerr(&mut self) -> DecerrW<'_, IenSpec> {
        DecerrW::new(self, 18)
    }
    #[doc = "Bit 19 - BUFDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn bufdatav(&mut self) -> BufdatavW<'_, IenSpec> {
        BufdatavW::new(self, 19)
    }
    #[doc = "Bit 20 - BUFLEVEL Interrupt Enable"]
    #[inline(always)]
    pub fn buflevel(&mut self) -> BuflevelW<'_, IenSpec> {
        BuflevelW::new(self, 20)
    }
    #[doc = "Bit 21 - BUFOF Interrupt Enable"]
    #[inline(always)]
    pub fn bufof(&mut self) -> BufofW<'_, IenSpec> {
        BufofW::new(self, 21)
    }
    #[doc = "Bit 22 - CNTOF Interrupt Enable"]
    #[inline(always)]
    pub fn cntof(&mut self) -> CntofW<'_, IenSpec> {
        CntofW::new(self, 22)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
