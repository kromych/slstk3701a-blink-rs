#[doc = "Register `STACKEDVLAN` reader"]
pub type R = crate::R<STACKEDVLAN_SPEC>;
#[doc = "Register `STACKEDVLAN` writer"]
pub type W = crate::W<STACKEDVLAN_SPEC>;
#[doc = "Field `MATCH` reader - User defined VLAN_TYPE field"]
pub type MATCH_R = crate::FieldReader<u16>;
#[doc = "Field `MATCH` writer - User defined VLAN_TYPE field"]
pub type MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENBPROCESSING` reader - Enable stacked VLAN processing mode"]
pub type ENBPROCESSING_R = crate::BitReader;
#[doc = "Field `ENBPROCESSING` writer - Enable stacked VLAN processing mode"]
pub type ENBPROCESSING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    pub fn enbprocessing(&self) -> ENBPROCESSING_R {
        ENBPROCESSING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field"]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<STACKEDVLAN_SPEC> {
        MATCH_W::new(self, 0)
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    #[must_use]
    pub fn enbprocessing(&mut self) -> ENBPROCESSING_W<STACKEDVLAN_SPEC> {
        ENBPROCESSING_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Stacked VLAN Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stackedvlan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stackedvlan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STACKEDVLAN_SPEC;
impl crate::RegisterSpec for STACKEDVLAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stackedvlan::R`](R) reader structure"]
impl crate::Readable for STACKEDVLAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stackedvlan::W`](W) writer structure"]
impl crate::Writable for STACKEDVLAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STACKEDVLAN to value 0"]
impl crate::Resettable for STACKEDVLAN_SPEC {
    const RESET_VALUE: u32 = 0;
}
