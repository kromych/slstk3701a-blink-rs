#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `CH0` reader - CH0 Interrupt Flag"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - CH1 Interrupt Flag"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH2` reader - CH2 Interrupt Flag"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH3` reader - CH3 Interrupt Flag"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH4` reader - CH4 Interrupt Flag"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH5` reader - CH5 Interrupt Flag"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH6` reader - CH6 Interrupt Flag"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH7` reader - CH7 Interrupt Flag"]
pub type Ch7R = crate::BitReader;
#[doc = "Field `CH8` reader - CH8 Interrupt Flag"]
pub type Ch8R = crate::BitReader;
#[doc = "Field `CH9` reader - CH9 Interrupt Flag"]
pub type Ch9R = crate::BitReader;
#[doc = "Field `CH10` reader - CH10 Interrupt Flag"]
pub type Ch10R = crate::BitReader;
#[doc = "Field `CH11` reader - CH11 Interrupt Flag"]
pub type Ch11R = crate::BitReader;
#[doc = "Field `CH12` reader - CH12 Interrupt Flag"]
pub type Ch12R = crate::BitReader;
#[doc = "Field `CH13` reader - CH13 Interrupt Flag"]
pub type Ch13R = crate::BitReader;
#[doc = "Field `CH14` reader - CH14 Interrupt Flag"]
pub type Ch14R = crate::BitReader;
#[doc = "Field `CH15` reader - CH15 Interrupt Flag"]
pub type Ch15R = crate::BitReader;
#[doc = "Field `SCANCOMPLETE` reader - SCANCOMPLETE Interrupt Flag"]
pub type ScancompleteR = crate::BitReader;
#[doc = "Field `DEC` reader - DEC Interrupt Flag"]
pub type DecR = crate::BitReader;
#[doc = "Field `DECERR` reader - DECERR Interrupt Flag"]
pub type DecerrR = crate::BitReader;
#[doc = "Field `BUFDATAV` reader - BUFDATAV Interrupt Flag"]
pub type BufdatavR = crate::BitReader;
#[doc = "Field `BUFLEVEL` reader - BUFLEVEL Interrupt Flag"]
pub type BuflevelR = crate::BitReader;
#[doc = "Field `BUFOF` reader - BUFOF Interrupt Flag"]
pub type BufofR = crate::BitReader;
#[doc = "Field `CNTOF` reader - CNTOF Interrupt Flag"]
pub type CntofR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CH0 Interrupt Flag"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Interrupt Flag"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Interrupt Flag"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Interrupt Flag"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4 Interrupt Flag"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5 Interrupt Flag"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6 Interrupt Flag"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7 Interrupt Flag"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8 Interrupt Flag"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9 Interrupt Flag"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10 Interrupt Flag"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11 Interrupt Flag"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CH12 Interrupt Flag"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CH13 Interrupt Flag"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CH14 Interrupt Flag"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CH15 Interrupt Flag"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCANCOMPLETE Interrupt Flag"]
    #[inline(always)]
    pub fn scancomplete(&self) -> ScancompleteR {
        ScancompleteR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DEC Interrupt Flag"]
    #[inline(always)]
    pub fn dec(&self) -> DecR {
        DecR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DECERR Interrupt Flag"]
    #[inline(always)]
    pub fn decerr(&self) -> DecerrR {
        DecerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BUFDATAV Interrupt Flag"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BufdatavR {
        BufdatavR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BUFLEVEL Interrupt Flag"]
    #[inline(always)]
    pub fn buflevel(&self) -> BuflevelR {
        BuflevelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BUFOF Interrupt Flag"]
    #[inline(always)]
    pub fn bufof(&self) -> BufofR {
        BufofR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CNTOF Interrupt Flag"]
    #[inline(always)]
    pub fn cntof(&self) -> CntofR {
        CntofR::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
