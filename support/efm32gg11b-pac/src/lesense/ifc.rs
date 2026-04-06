#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `CH0` writer - Clear CH0 Interrupt Flag"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Clear CH1 Interrupt Flag"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Clear CH2 Interrupt Flag"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Clear CH3 Interrupt Flag"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Clear CH4 Interrupt Flag"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Clear CH5 Interrupt Flag"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Clear CH6 Interrupt Flag"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Clear CH7 Interrupt Flag"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Clear CH8 Interrupt Flag"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Clear CH9 Interrupt Flag"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Clear CH10 Interrupt Flag"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Clear CH11 Interrupt Flag"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - Clear CH12 Interrupt Flag"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - Clear CH13 Interrupt Flag"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - Clear CH14 Interrupt Flag"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - Clear CH15 Interrupt Flag"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCOMPLETE` writer - Clear SCANCOMPLETE Interrupt Flag"]
pub type ScancompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEC` writer - Clear DEC Interrupt Flag"]
pub type DecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECERR` writer - Clear DECERR Interrupt Flag"]
pub type DecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFDATAV` writer - Clear BUFDATAV Interrupt Flag"]
pub type BufdatavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFLEVEL` writer - Clear BUFLEVEL Interrupt Flag"]
pub type BuflevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOF` writer - Clear BUFOF Interrupt Flag"]
pub type BufofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOF` writer - Clear CNTOF Interrupt Flag"]
pub type CntofW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear CH0 Interrupt Flag"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, IfcSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear CH1 Interrupt Flag"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, IfcSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear CH2 Interrupt Flag"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, IfcSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear CH3 Interrupt Flag"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, IfcSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear CH4 Interrupt Flag"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, IfcSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear CH5 Interrupt Flag"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, IfcSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear CH6 Interrupt Flag"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, IfcSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear CH7 Interrupt Flag"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, IfcSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear CH8 Interrupt Flag"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<'_, IfcSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear CH9 Interrupt Flag"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<'_, IfcSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear CH10 Interrupt Flag"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<'_, IfcSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear CH11 Interrupt Flag"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<'_, IfcSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear CH12 Interrupt Flag"]
    #[inline(always)]
    pub fn ch12(&mut self) -> Ch12W<'_, IfcSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear CH13 Interrupt Flag"]
    #[inline(always)]
    pub fn ch13(&mut self) -> Ch13W<'_, IfcSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear CH14 Interrupt Flag"]
    #[inline(always)]
    pub fn ch14(&mut self) -> Ch14W<'_, IfcSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear CH15 Interrupt Flag"]
    #[inline(always)]
    pub fn ch15(&mut self) -> Ch15W<'_, IfcSpec> {
        Ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear SCANCOMPLETE Interrupt Flag"]
    #[inline(always)]
    pub fn scancomplete(&mut self) -> ScancompleteW<'_, IfcSpec> {
        ScancompleteW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear DEC Interrupt Flag"]
    #[inline(always)]
    pub fn dec(&mut self) -> DecW<'_, IfcSpec> {
        DecW::new(self, 17)
    }
    #[doc = "Bit 18 - Clear DECERR Interrupt Flag"]
    #[inline(always)]
    pub fn decerr(&mut self) -> DecerrW<'_, IfcSpec> {
        DecerrW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear BUFDATAV Interrupt Flag"]
    #[inline(always)]
    pub fn bufdatav(&mut self) -> BufdatavW<'_, IfcSpec> {
        BufdatavW::new(self, 19)
    }
    #[doc = "Bit 20 - Clear BUFLEVEL Interrupt Flag"]
    #[inline(always)]
    pub fn buflevel(&mut self) -> BuflevelW<'_, IfcSpec> {
        BuflevelW::new(self, 20)
    }
    #[doc = "Bit 21 - Clear BUFOF Interrupt Flag"]
    #[inline(always)]
    pub fn bufof(&mut self) -> BufofW<'_, IfcSpec> {
        BufofW::new(self, 21)
    }
    #[doc = "Bit 22 - Clear CNTOF Interrupt Flag"]
    #[inline(always)]
    pub fn cntof(&mut self) -> CntofW<'_, IfcSpec> {
        CntofW::new(self, 22)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
