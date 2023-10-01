#[doc = "Register `PEEK` reader"]
pub type R = crate::R<PEEK_SPEC>;
#[doc = "Field `CH0VAL` reader - Channel 0 Current Value"]
pub type CH0VAL_R = crate::BitReader;
#[doc = "Field `CH1VAL` reader - Channel 1 Current Value"]
pub type CH1VAL_R = crate::BitReader;
#[doc = "Field `CH2VAL` reader - Channel 2 Current Value"]
pub type CH2VAL_R = crate::BitReader;
#[doc = "Field `CH3VAL` reader - Channel 3 Current Value"]
pub type CH3VAL_R = crate::BitReader;
#[doc = "Field `CH4VAL` reader - Channel 4 Current Value"]
pub type CH4VAL_R = crate::BitReader;
#[doc = "Field `CH5VAL` reader - Channel 5 Current Value"]
pub type CH5VAL_R = crate::BitReader;
#[doc = "Field `CH6VAL` reader - Channel 6 Current Value"]
pub type CH6VAL_R = crate::BitReader;
#[doc = "Field `CH7VAL` reader - Channel 7 Current Value"]
pub type CH7VAL_R = crate::BitReader;
#[doc = "Field `CH8VAL` reader - Channel 8 Current Value"]
pub type CH8VAL_R = crate::BitReader;
#[doc = "Field `CH9VAL` reader - Channel 9 Current Value"]
pub type CH9VAL_R = crate::BitReader;
#[doc = "Field `CH10VAL` reader - Channel 10 Current Value"]
pub type CH10VAL_R = crate::BitReader;
#[doc = "Field `CH11VAL` reader - Channel 11 Current Value"]
pub type CH11VAL_R = crate::BitReader;
#[doc = "Field `CH12VAL` reader - Channel 12 Current Value"]
pub type CH12VAL_R = crate::BitReader;
#[doc = "Field `CH13VAL` reader - Channel 13 Current Value"]
pub type CH13VAL_R = crate::BitReader;
#[doc = "Field `CH14VAL` reader - Channel 14 Current Value"]
pub type CH14VAL_R = crate::BitReader;
#[doc = "Field `CH15VAL` reader - Channel 15 Current Value"]
pub type CH15VAL_R = crate::BitReader;
#[doc = "Field `CH16VAL` reader - Channel 16 Current Value"]
pub type CH16VAL_R = crate::BitReader;
#[doc = "Field `CH17VAL` reader - Channel 17 Current Value"]
pub type CH17VAL_R = crate::BitReader;
#[doc = "Field `CH18VAL` reader - Channel 18 Current Value"]
pub type CH18VAL_R = crate::BitReader;
#[doc = "Field `CH19VAL` reader - Channel 19 Current Value"]
pub type CH19VAL_R = crate::BitReader;
#[doc = "Field `CH20VAL` reader - Channel 20 Current Value"]
pub type CH20VAL_R = crate::BitReader;
#[doc = "Field `CH21VAL` reader - Channel 21 Current Value"]
pub type CH21VAL_R = crate::BitReader;
#[doc = "Field `CH22VAL` reader - Channel 22 Current Value"]
pub type CH22VAL_R = crate::BitReader;
#[doc = "Field `CH23VAL` reader - Channel 23 Current Value"]
pub type CH23VAL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Current Value"]
    #[inline(always)]
    pub fn ch0val(&self) -> CH0VAL_R {
        CH0VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Current Value"]
    #[inline(always)]
    pub fn ch1val(&self) -> CH1VAL_R {
        CH1VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Current Value"]
    #[inline(always)]
    pub fn ch2val(&self) -> CH2VAL_R {
        CH2VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Current Value"]
    #[inline(always)]
    pub fn ch3val(&self) -> CH3VAL_R {
        CH3VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Current Value"]
    #[inline(always)]
    pub fn ch4val(&self) -> CH4VAL_R {
        CH4VAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Current Value"]
    #[inline(always)]
    pub fn ch5val(&self) -> CH5VAL_R {
        CH5VAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Current Value"]
    #[inline(always)]
    pub fn ch6val(&self) -> CH6VAL_R {
        CH6VAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Current Value"]
    #[inline(always)]
    pub fn ch7val(&self) -> CH7VAL_R {
        CH7VAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Current Value"]
    #[inline(always)]
    pub fn ch8val(&self) -> CH8VAL_R {
        CH8VAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Current Value"]
    #[inline(always)]
    pub fn ch9val(&self) -> CH9VAL_R {
        CH9VAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Current Value"]
    #[inline(always)]
    pub fn ch10val(&self) -> CH10VAL_R {
        CH10VAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Current Value"]
    #[inline(always)]
    pub fn ch11val(&self) -> CH11VAL_R {
        CH11VAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Current Value"]
    #[inline(always)]
    pub fn ch12val(&self) -> CH12VAL_R {
        CH12VAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Current Value"]
    #[inline(always)]
    pub fn ch13val(&self) -> CH13VAL_R {
        CH13VAL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Current Value"]
    #[inline(always)]
    pub fn ch14val(&self) -> CH14VAL_R {
        CH14VAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Current Value"]
    #[inline(always)]
    pub fn ch15val(&self) -> CH15VAL_R {
        CH15VAL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 16 Current Value"]
    #[inline(always)]
    pub fn ch16val(&self) -> CH16VAL_R {
        CH16VAL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 17 Current Value"]
    #[inline(always)]
    pub fn ch17val(&self) -> CH17VAL_R {
        CH17VAL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 18 Current Value"]
    #[inline(always)]
    pub fn ch18val(&self) -> CH18VAL_R {
        CH18VAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 19 Current Value"]
    #[inline(always)]
    pub fn ch19val(&self) -> CH19VAL_R {
        CH19VAL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 20 Current Value"]
    #[inline(always)]
    pub fn ch20val(&self) -> CH20VAL_R {
        CH20VAL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 21 Current Value"]
    #[inline(always)]
    pub fn ch21val(&self) -> CH21VAL_R {
        CH21VAL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 22 Current Value"]
    #[inline(always)]
    pub fn ch22val(&self) -> CH22VAL_R {
        CH22VAL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 23 Current Value"]
    #[inline(always)]
    pub fn ch23val(&self) -> CH23VAL_R {
        CH23VAL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "PRS Channel Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEEK_SPEC;
impl crate::RegisterSpec for PEEK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peek::R`](R) reader structure"]
impl crate::Readable for PEEK_SPEC {}
#[doc = "`reset()` method sets PEEK to value 0"]
impl crate::Resettable for PEEK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
