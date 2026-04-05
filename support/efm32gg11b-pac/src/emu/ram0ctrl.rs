#[doc = "Register `RAM0CTRL` reader"]
pub type R = crate::R<Ram0ctrlSpec>;
#[doc = "Register `RAM0CTRL` writer"]
pub type W = crate::W<Ram0ctrlSpec>;
#[doc = "RAM0 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rampowerdown {
    #[doc = "0: None of the RAM blocks powered down"]
    None = 0,
    #[doc = "64: Power down RAM block 7 and above"]
    Blk7 = 64,
    #[doc = "96: Power down RAM block 6 and above"]
    Blk6to7 = 96,
    #[doc = "112: Power down RAM block 5 and above"]
    Blk5to7 = 112,
    #[doc = "120: Power down RAM blocks 4 and above"]
    Blk4to7 = 120,
    #[doc = "124: Power down RAM blocks 3 and above"]
    Blk3to7 = 124,
    #[doc = "126: Power down RAM blocks 2 and above"]
    Blk2to7 = 126,
    #[doc = "127: Power down RAM blocks 1 and above"]
    Blk1to7 = 127,
}
impl From<Rampowerdown> for u8 {
    #[inline(always)]
    fn from(variant: Rampowerdown) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rampowerdown {
    type Ux = u8;
}
impl crate::IsEnum for Rampowerdown {}
#[doc = "Field `RAMPOWERDOWN` reader - RAM0 Blockset Power-down"]
pub type RampowerdownR = crate::FieldReader<Rampowerdown>;
impl RampowerdownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rampowerdown> {
        match self.bits {
            0 => Some(Rampowerdown::None),
            64 => Some(Rampowerdown::Blk7),
            96 => Some(Rampowerdown::Blk6to7),
            112 => Some(Rampowerdown::Blk5to7),
            120 => Some(Rampowerdown::Blk4to7),
            124 => Some(Rampowerdown::Blk3to7),
            126 => Some(Rampowerdown::Blk2to7),
            127 => Some(Rampowerdown::Blk1to7),
            _ => None,
        }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rampowerdown::None
    }
    #[doc = "Power down RAM block 7 and above"]
    #[inline(always)]
    pub fn is_blk7(&self) -> bool {
        *self == Rampowerdown::Blk7
    }
    #[doc = "Power down RAM block 6 and above"]
    #[inline(always)]
    pub fn is_blk6to7(&self) -> bool {
        *self == Rampowerdown::Blk6to7
    }
    #[doc = "Power down RAM block 5 and above"]
    #[inline(always)]
    pub fn is_blk5to7(&self) -> bool {
        *self == Rampowerdown::Blk5to7
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline(always)]
    pub fn is_blk4to7(&self) -> bool {
        *self == Rampowerdown::Blk4to7
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline(always)]
    pub fn is_blk3to7(&self) -> bool {
        *self == Rampowerdown::Blk3to7
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline(always)]
    pub fn is_blk2to7(&self) -> bool {
        *self == Rampowerdown::Blk2to7
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline(always)]
    pub fn is_blk1to7(&self) -> bool {
        *self == Rampowerdown::Blk1to7
    }
}
#[doc = "Field `RAMPOWERDOWN` writer - RAM0 Blockset Power-down"]
pub type RampowerdownW<'a, REG> = crate::FieldWriter<'a, REG, 7, Rampowerdown>;
impl<'a, REG> RampowerdownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::None)
    }
    #[doc = "Power down RAM block 7 and above"]
    #[inline(always)]
    pub fn blk7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk7)
    }
    #[doc = "Power down RAM block 6 and above"]
    #[inline(always)]
    pub fn blk6to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk6to7)
    }
    #[doc = "Power down RAM block 5 and above"]
    #[inline(always)]
    pub fn blk5to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk5to7)
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline(always)]
    pub fn blk4to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk4to7)
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline(always)]
    pub fn blk3to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk3to7)
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline(always)]
    pub fn blk2to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk2to7)
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline(always)]
    pub fn blk1to7(self) -> &'a mut crate::W<REG> {
        self.variant(Rampowerdown::Blk1to7)
    }
}
impl R {
    #[doc = "Bits 0:6 - RAM0 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RampowerdownR {
        RampowerdownR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RAM0 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&mut self) -> RampowerdownW<'_, Ram0ctrlSpec> {
        RampowerdownW::new(self, 0)
    }
}
#[doc = "Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram0ctrlSpec;
impl crate::RegisterSpec for Ram0ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0ctrl::R`](R) reader structure"]
impl crate::Readable for Ram0ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ram0ctrl::W`](W) writer structure"]
impl crate::Writable for Ram0ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAM0CTRL to value 0"]
impl crate::Resettable for Ram0ctrlSpec {}
